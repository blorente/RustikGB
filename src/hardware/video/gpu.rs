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

use std::fmt;
use rand;

pub struct GPU {

    pub tile_data: TileSet,
    tile_maps: PLAIN_RAM,
    sprite_oam: PLAIN_RAM,

    lcdc_mode: LCDCMode,
    mode_cycles: u32,

    // IO Registers for the GPU
    lcd_control:    Register<u8>,  
    lcd_status:     Register<u8>,   
    scroll_y:       Register<u8>,
    scroll_x:       Register<u8>,
    ly_coord:       Register<u8>,
    lyc_compare:    Register<u8>,
    dma_start:      Register<u8>,
    bg_palette:     Register<u8>,
    obj_palette_1:  Register<u8>,
    obj_palette_2:  Register<u8>,
    window_y:       Register<u8>,
    window_x:       Register<u8>,

    debug_color:    [u8; 4]
}

#[derive(PartialEq, Debug)]
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
            sprite_oam: PLAIN_RAM::new(SPRITE_OAM_START, SPRITE_OAM_END),

            lcdc_mode: LCDCMode::OAM,
            mode_cycles: 0x0,

            lcd_control:    Register::new(0x00),  
            lcd_status:     Register::new(0x00),   
            scroll_y:       Register::new(0x00),
            scroll_x:       Register::new(0x00),
            ly_coord:       Register::new(0x00),
            lyc_compare:    Register::new(0x00),
            dma_start:      Register::new(0x00),
            bg_palette:     Register::new(0x00),
            obj_palette_1:  Register::new(0x00),
            obj_palette_2:  Register::new(0x00),
            window_y:       Register::new(0x00),
            window_x:       Register::new(0x00),

            debug_color:    [0, 0, 0, 255]
        }
    }

    pub fn step(&mut self, cycles: u32, screen: &mut Screen) {
        // TODO: Uncomment when more advanced with tetris, See if it bootstrap really switches it off
        //screen.turn_on_off(self.lcd_control.is_bit_set(B_LCD_DISPLAY_ENABLED));
        
        //println!("Mode: {} Cycles: {}", self.lcdc_mode, self.mode_cycles);
        self.update_mode(cycles, screen);  

        /*
        if self.ly_coord.r() == 144 {
            self.debug_color = [
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
                255
            ]
        } else {        
            let y = self.ly_coord.r();
            screen.set_pixel(0, y, self.debug_color);
            screen.set_pixel(1, y, self.debug_color);
            screen.set_pixel(2, y, self.debug_color);
            screen.set_pixel(3, y, self.debug_color);
            screen.set_pixel(4, y, self.debug_color);
            screen.set_pixel(5, y, self.debug_color);
            screen.set_pixel(6, y, self.debug_color);
        }
        */
    }    

    fn update_mode(&mut self, cycles: u32, screen: &mut Screen) {
        self.mode_cycles += cycles;
        match self.lcdc_mode {
            LCDCMode::HBLANK => {                
                if self.need_change_mode(HBLANK_CYCLES) {
                    self.increase_line();
                    let y = self.ly_coord.r();
                    if self.ly_coord.r() == VBLANK_START_LINE {
                        self.change_mode_and_interrupt(LCDCMode::VBLANK);
                    } else {                        
                        self.render_scan_line(screen);
                        self.change_mode_and_interrupt(LCDCMode::OAM);
                    }
                    self.mode_cycles -= HBLANK_CYCLES;
                }
            }
            LCDCMode::VBLANK => {
                if self.ly_coord.r() == VBLANK_END_LINE {
                    self.change_mode_and_interrupt(LCDCMode::OAM);                   
                }
                if self.mode_cycles > CYCLES_PER_LINE {
                    self.increase_line();
                    self.mode_cycles -= CYCLES_PER_LINE;
                }
            }
            LCDCMode::OAM => {
                if self.need_change_mode(OAM_CYCLES) {
                    self.change_mode_and_interrupt(LCDCMode::VRAM);
                    self.mode_cycles -= OAM_CYCLES;
                }
            }
            LCDCMode::VRAM => {
                if self.need_change_mode(VRAM_CYCLES) {
                    self.change_mode_and_interrupt(LCDCMode::HBLANK);
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
        if self.ly_coord.r() == self.lyc_compare.r() {
            self.lcd_status.set_bit(B_LYC_COINCIDENCE_INTERRUPT, true);
            // TODO: Request a STAT interrupt
        }
    }

    fn change_mode_and_interrupt(&mut self, target: LCDCMode) {
        //println!("Mode changed to {}", target); 
        self.lcdc_mode = target;
        // TODO: Interrupts go here
    }

    fn render_scan_line(&mut self, screen: &mut Screen) {
        self.render_background_line(screen);
    }

    fn render_background_line(&mut self, screen: &mut Screen) { 
        let background_tile_map_start = 
            if self.lcd_control.is_bit_set(B_BG_TILE_MAP_SELECT) {
                TILE_MAP_1_START
            } else {
                TILE_MAP_0_START
            };
        let signed_tile_maps: bool;
        let background_tile_data_start = 
            if self.lcd_control.is_bit_set(B_BG_WIN_TILE_DATA_SELECT) {
                signed_tile_maps = false;
                BG_WIN_TILE_DATA_1_START
            } else {
                signed_tile_maps = true;
                BG_WIN_TILE_DATA_0_START
            };

        let tile_y = (((self.ly_coord.r() + self.scroll_y.r()) / 8) % 32) as u16;
        let tile_offset_y = ((self.ly_coord.r() + self.scroll_y.r()) % 8) as u16;

        for x in 0..SCREEN_WIDTH {            
            let tile_offset_x = ((self.scroll_x.r() + x as u8) % 8) as u16;
            let tile_x = ((self.scroll_x.r().wrapping_add(x as u8) / 8) % 32) as u16;
            let tile_index = self.tile_maps.read_byte(background_tile_map_start + (tile_y * 32) + tile_x);
            let tile_address =              
                (if signed_tile_maps {tile_index as u16} 
                else {(tile_index as i8 as i16 + 128) as u16});
            let tile = self.tile_data.tiles[tile_address as usize];
            let y = self.ly_coord.r();

            let color = PALETTE_PINKU[self.tile_data.get_pixel(&tile, tile_offset_y as u8, tile_offset_x as u8) as usize];
            
            //println!("Get pixel ({}, {}). Color: {:?} Tile: {:4X}", tile_offset_x, tile_offset_y, color, tile_index);
            screen.set_pixel(x as u8, y, color);
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
                DMA_START_ADDR          => {panic!("DMA is write only");}
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
                DMA_START_ADDR          => {panic!("DMA Transfer disabled until it's implemented")}
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
        || match addr {            
            LCD_CONTROL_ADDR        => {true}
            LCD_STATUS_ADDR         => {true}
            SCROLL_Y_ADDR           => {true}
            SCROLL_X_ADDR           => {true}
            LY_COORD_ADDR           => {true}
            LYC_COMPLARE_ADDR       => {true}
            DMA_START_ADDR          => {true}
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