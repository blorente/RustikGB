use hardware::memory::plain_ram::PLAIN_RAM;
use hardware::memory::memory_region::MemoryRegion;
use hardware::memory::memory_region::BitAccess;
use hardware::registers::Register;
use hardware::video::screen::Screen;
use hardware::video::screen::SCREEN_WIDTH;
use hardware::video::screen::SCREEN_HEIGHT;
use hardware::hex_print;
use hardware::video::gpu_constants::*;
use hardware::video::tile_set::TileSet;
use hardware::video::tile_set::Tile;
use hardware::interrupts::Interrupts;
use hardware::interrupts::InterruptType;
use hardware::video::sprites::SpriteOAM;
use hardware::video::sprites::Sprite;

use std::fmt;
use rand;

pub struct GPU {

    pub tile_data: TileSet,
    tile_maps: PLAIN_RAM,
    sprite_oam: SpriteOAM,

    lcdc_mode: LCDCMode,
    mode_cycles: u32,

    // IO Registers for the GPU
    lcd_control:    Register<u8>,  
    lcd_status:     Register<u8>,   
    scroll_y:       Register<u8>,
    scroll_x:       Register<u8>,
    ly_coord:       Register<u8>,
    lyc_compare:    Register<u8>,
    bg_palette:     Register<u8>,
    obj_palette_1:  Register<u8>,
    obj_palette_2:  Register<u8>,
    window_y:       Register<u8>,
    window_x:       Register<u8>,

    debug_color:    [u8; 4]
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum LCDCMode {
    HBLANK  = 0b00,    
    VBLANK  = 0b01,    
    OAM     = 0b10,
    VRAM    = 0b11,
}


impl fmt::Display for LCDCMode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        
        write!(fmt, "{:?}", self)
    }
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            tile_data: TileSet::new(),
            tile_maps: PLAIN_RAM::new(TILE_MAPS_START, TILE_MAPS_END),
            sprite_oam: SpriteOAM::new(),

            lcdc_mode: LCDCMode::OAM,
            mode_cycles: 0x0,

            lcd_control:    Register::new(0x00),  
            lcd_status:     Register::new(0x00),   
            scroll_y:       Register::new(0x00),
            scroll_x:       Register::new(0x00),
            ly_coord:       Register::new(0x00),
            lyc_compare:    Register::new(0x00),
            bg_palette:     Register::new(0x00),
            obj_palette_1:  Register::new(0x00),
            obj_palette_2:  Register::new(0x00),
            window_y:       Register::new(0x00),
            window_x:       Register::new(0x00),

            debug_color:    [0, 0, 0, 255]
        }
    }

    pub fn step(&mut self, cycles: u32, screen: &mut Screen, interrupt_handler: &mut Interrupts) {
        // If the display is not enabled, don't render
        //if !self.lcd_control.is_bit_set(B_LCD_DISPLAY_ENABLED) {return}

        //println!("Mode: {} Cycles: {}", self.lcdc_mode, self.mode_cycles);
        self.update_mode(cycles, screen, interrupt_handler); 
        self.generate_interrupts(interrupt_handler); 
    }    

    fn update_mode(&mut self, cycles: u32, screen: &mut Screen, interrupt_handler: &mut Interrupts) {
        self.mode_cycles += cycles;
        match self.lcdc_mode {
            LCDCMode::HBLANK => {                
                if self.need_change_mode(HBLANK_CYCLES) {
                    self.increase_line();
                    let y = self.ly_coord.r();
                    if self.ly_coord.r() == VBLANK_START_LINE {
                        self.change_mode_and_interrupt(LCDCMode::VBLANK, interrupt_handler);
                    } else {                        
                        self.render_scan_line(screen);
                        self.change_mode_and_interrupt(LCDCMode::OAM, interrupt_handler);
                    }
                    self.mode_cycles -= HBLANK_CYCLES;
                }
            }
            LCDCMode::VBLANK => {
                if self.ly_coord.r() == VBLANK_END_LINE {
                    self.change_mode_and_interrupt(LCDCMode::OAM, interrupt_handler);                   
                }
                if self.mode_cycles > CYCLES_PER_LINE {
                    self.increase_line();
                    self.mode_cycles -= CYCLES_PER_LINE;
                }
            }
            LCDCMode::OAM => {
                if self.need_change_mode(OAM_CYCLES) {
                    self.change_mode_and_interrupt(LCDCMode::VRAM, interrupt_handler);
                    self.mode_cycles -= OAM_CYCLES;
                }
            }
            LCDCMode::VRAM => {
                if self.need_change_mode(VRAM_CYCLES) {
                    self.change_mode_and_interrupt(LCDCMode::HBLANK, interrupt_handler);
                    self.mode_cycles -= VRAM_CYCLES;
                }
            }
        }
    }

    fn need_change_mode(&mut self, max_cycles: u32) -> bool {        
        self.mode_cycles > max_cycles
    }

    fn increase_line(&mut self) {        
        let line = (self.ly_coord.r() + 1) % (VBLANK_END_LINE + 1);
        self.ly_coord.w(line);

        //println!("{} Line increased to {}", self.lcdc_mode, self.ly_coord.r());       

        // Check the value of LY with LYC register, and request interrupts if necessary
        self.lcd_status.set_bit(B_LYC_COINCIDENCE_FLAG, self.ly_coord.r() == self.lyc_compare.r());        
    }

    fn change_mode_and_interrupt(&mut self, target: LCDCMode, interrupt_handler: &mut Interrupts) {
        //println!("Mode changed to {}", target); 
        self.lcdc_mode = target;
        if self.lcdc_mode == LCDCMode::VBLANK {
            interrupt_handler.set_interrupt(InterruptType::VBlank);
        }
        let mode = target as u8;
        self.lcd_status.set_bit(B_LCDC_STATUS_0_FLAG, (mode & 0b01) > 0);
        self.lcd_status.set_bit(B_LCDC_STATUS_1_FLAG, (mode & 0b10) > 0);
    }

    fn generate_interrupts(&self, interrupt_handler: &mut Interrupts) {
        match self.lcdc_mode {
            LCDCMode::HBLANK => { if self.lcd_status.is_bit_set(B_HBLANK_INTERRUPT) {interrupt_handler.set_interrupt(InterruptType::LCDC)}}
            LCDCMode::VBLANK => { if self.lcd_status.is_bit_set(B_VBLANK_INTERRUPT) {interrupt_handler.set_interrupt(InterruptType::LCDC)}}
            LCDCMode::OAM    => { if self.lcd_status.is_bit_set(B_OAM_INTERRUPT)    {interrupt_handler.set_interrupt(InterruptType::LCDC)}}
            _ => {}
        }
        if self.lcd_status.is_bit_set(B_LYC_COINCIDENCE_INTERRUPT) 
            && self.lcd_status.is_bit_set(B_LYC_COINCIDENCE_FLAG) {
            interrupt_handler.set_interrupt(InterruptType::LCDC);
        }
    }

    fn render_scan_line(&mut self, screen: &mut Screen) {  
        self.render_background_line(screen);
        self.render_sprites_in_line(screen);
    }

    fn render_background_line(&mut self, screen: &mut Screen) { 
        let background_tile_map_start = 
            if self.lcd_control.is_bit_set(B_BG_TILE_MAP_SELECT) {
                TILE_MAP_1_START
            } else {
                TILE_MAP_0_START
            };           

        let tile_y = (((self.ly_coord.r() + self.scroll_y.r()) / 8) % 32) as u16;
        let tile_offset_y = ((self.ly_coord.r() + self.scroll_y.r()) % 8) as u16;
 
        for x in 0..SCREEN_WIDTH {            
            let tile_offset_x = ((self.scroll_x.r() + x as u8) % 8) as u16;
            let tile_x = ((self.scroll_x.r().wrapping_add(x as u8) / 8) % 32) as u16;
            let tile_index = self.tile_maps.read_byte(background_tile_map_start + (tile_y * 32) + tile_x);
            let tile = self.tile_data.tiles[tile_index as usize];
            let y = self.ly_coord.r();

            let color = PALETTE_IN_USE[self.tile_data.get_pixel(&tile, tile_offset_y as u8, tile_offset_x as u8) as usize];
            
            //println!("Get pixel ({}, {}). Color: {:?} Tile: {:4X}", tile_offset_x, tile_offset_y, color, tile_index);
            screen.set_pixel(x as u8, y, color);
        }
    }

    fn render_sprites_in_line(&mut self, screen: &mut Screen) {
        for sprite in &self.sprite_oam.sprites {
            if sprite.in_valid_position(self.ly_coord.r()) {   
                let line = sprite.coord_y - self.ly_coord.r() - 16;            
                let tile_offset_y = 
                    if sprite.flip_y {(7 - line)}
                    else {line};
                let start_y = sprite.coord_y - 16;
                let start_x = sprite.coord_x - 8;
                let tile = self.tile_data.tiles[sprite.data_tile as usize];
                for pixel in 0..8 {
                    let tile_offset_x = if sprite.flip_x {7 - pixel} else {pixel};
                    let color = PALETTE_IN_USE[self.tile_data.get_pixel(&tile, tile_offset_y as u8, tile_offset_x as u8) as usize];
                    screen.set_pixel(start_x + pixel as u8, start_y + line, color);
                }                
            }
        }
    }
}

impl MemoryRegion for GPU {
    fn read_byte(&self, addr: u16) -> u8 {
        if self.tile_data.in_region(addr) {
            self.tile_data.read_byte(addr)
        } else if self.tile_maps.in_region(addr) {
            self.tile_maps.read_byte(addr)
        } else if self.sprite_oam.in_region(addr) {
            self.sprite_oam.read_byte(addr)
        } else {
            match addr {            
                LCD_CONTROL_ADDR        => {self.lcd_control.r()}
                LCD_STATUS_ADDR         => {self.lcd_status.r()}
                SCROLL_Y_ADDR           => {self.scroll_y.r()}
                SCROLL_X_ADDR           => {self.scroll_x.r()}
                LY_COORD_ADDR           => {self.ly_coord.r()}
                LYC_COMPLARE_ADDR       => {self.lyc_compare.r()}
                BG_PALLETE_ADDR         => {self.bg_palette.r()}
                OBJECT_PALETTE_1_ADDR   => {self.obj_palette_1.r()}
                OBJECT_PALETTE_2_ADDR   => {self.obj_palette_2.r()}
                WINDOW_Y_ADDR           => {self.window_y.r()}
                WINDOW_X_ADDR           => {self.window_x.r()}
                _ => {panic!("GPU Can't access memory location {:04X}", addr);}
            }
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        if self.tile_data.in_region(addr) {
            self.tile_data.write_byte(addr, val);
        } else if self.tile_maps.in_region(addr) {
            self.tile_maps.write_byte(addr, val);
        } else if self.sprite_oam.in_region(addr) {
            self.sprite_oam.write_byte(addr, val)
        } else {
            match addr {            
                LCD_CONTROL_ADDR        => {self.lcd_control.w(val);} 
                LCD_STATUS_ADDR         => {self.lcd_status.w(val);}
                SCROLL_Y_ADDR           => {self.scroll_y.w(val);}
                SCROLL_X_ADDR           => {self.scroll_x.w(val);}
                LY_COORD_ADDR           => {self.ly_coord.w(0x00);}
                LYC_COMPLARE_ADDR       => {self.lyc_compare.w(val);}
                BG_PALLETE_ADDR         => {self.bg_palette.w(val);}
                OBJECT_PALETTE_1_ADDR   => {self.obj_palette_1.w(val);}
                OBJECT_PALETTE_2_ADDR   => {self.obj_palette_2.w(val);}
                WINDOW_Y_ADDR           => {self.window_y.w(val);}
                WINDOW_X_ADDR           => {self.window_x.w(val);}
                _ => {panic!("GPU Can't access memory location {:04X}", addr);}
            }
        }
    }

    fn in_region(&self, addr: u16) -> bool {
        self.tile_data.in_region(addr) 
        || self.tile_maps.in_region(addr)
        || self.sprite_oam.in_region(addr)
        || match addr {            
            LCD_CONTROL_ADDR        => {true}
            LCD_STATUS_ADDR         => {true}
            SCROLL_Y_ADDR           => {true}
            SCROLL_X_ADDR           => {true}
            LY_COORD_ADDR           => {true}
            LYC_COMPLARE_ADDR       => {true}
            BG_PALLETE_ADDR         => {true}
            OBJECT_PALETTE_1_ADDR   => {true}
            OBJECT_PALETTE_2_ADDR   => {true}
            WINDOW_Y_ADDR           => {true}
            WINDOW_X_ADDR           => {true}
            _ => {false}
        }
    }

    fn start(&self) -> u16 {
        panic!("GPU Doesn't have a real 'start()'");
    }

    fn end(&self) -> u16 {
        panic!("GPU Doesn't have a real 'end()'");
    }
}