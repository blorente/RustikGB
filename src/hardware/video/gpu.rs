use hardware::memory::plain_ram::PLAIN_RAM;
use hardware::memory::memory_region::MemoryRegion;
use hardware::memory::memory_region::BitAccess;
use hardware::registers::Register;
use hardware::video::screen::Screen;
use hardware::video::screen::SCREEN_WIDTH;
use hardware::video::screen::SCREEN_HEIGHT;
use hardware::hex_print;

use std::fmt;
use rand;

const SPRITE_OAM_START              : u16 = 0xFE00;
const SPRITE_OAM_END                : u16 = 0xFE9F;

// The VRAM is subdivided into sections.
// Still, it is useful to know the boundaries
// just for interfacing purpouses
const VRAM_START                    : u16 = 0x8000;
const VRAM_END                      : u16 = 0x9FFF;
// Granular VRAM regions
const BG_TILE_MAP_1_START           : u16 = 0x9800;
const BG_TILE_MAP_2_START           : u16 = 0x9C00;


const LCD_CONTROL_ADDR              : u16 = 0xFF40;
const LCD_STATUS_ADDR               : u16 = 0xFF41;
const SCROLL_Y_ADDR                 : u16 = 0xFF42;
const SCROLL_X_ADDR                 : u16 = 0xFF43;
const LY_COORD_ADDR                 : u16 = 0xFF44;
const LYC_COMPLARE_ADDR             : u16 = 0xFF45;
const DMA_START_ADDR                : u16 = 0xFF46;
const BG_PALLETE_ADDR               : u16 = 0xFF47;
const OBJECT_PALETTE_1_ADDR         : u16 = 0xFF48;
const OBJECT_PALETTE_2_ADDR         : u16 = 0xFF49;
const WINDOW_Y_ADDR                 : u16 = 0xFF4A;
const WINDOW_X_ADDR                 : u16 = 0xFF4B;

// Timing stuff
const CYCLES_PER_LINE               : u32 =  456;
const HBLANK_CYCLES                 : u32 =  204;
const VRAM_CYCLES                   : u32 =  172;
const OAM_CYCLES                    : u32 =   80;
const VBLANK_CYCLES                 : u32 = 4560;

const VBLANK_START_LINE             : u8  = 144;
const VBLANK_END_LINE               : u8  = 153;

// Relevant bits
// LCD Control Register
const B_LCD_DISPLAY_ENABLED         : u8 = 7;
const B_BG_WIN_TILE_DATA_SELECT     : u8 = 4;
const B_BG_TILE_MAP_SELECT          : u8 = 3;

// LCD Status Register
const B_LYC_COINCIDENCE_INTERRUPT   : u8 = 6;

pub struct GPU {

    vram: PLAIN_RAM,
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

    debug_color:    [u8; 3]
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
            vram: PLAIN_RAM::new(VRAM_START, VRAM_END),
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

            debug_color:    [0; 3]
        }
    }

    pub fn step(&mut self, cycles: u32, screen: &mut Screen) {
        // TODO: Uncomment when more advanced with tetris, See if it bootstrap really switches it off
        //screen.turn_on_off(self.lcd_control.is_bit_set(B_LCD_DISPLAY_ENABLED));
        
        //println!("Mode: {} Cycles: {}", self.lcdc_mode, self.mode_cycles);
        self.update_mode(cycles);  

        match self.lcdc_mode {
            LCDCMode::VBLANK => {
                if self.ly_coord.r() == 154 {
                    self.debug_color = [
                        rand::random::<u8>(),
                        rand::random::<u8>(),
                        rand::random::<u8>()
                    ]
                }
            }
            _ => {
                let y = self.ly_coord.r();
                screen.set_pixel(y, 0, self.debug_color);
                screen.set_pixel(y, 1, self.debug_color);
                screen.set_pixel(y, 2, self.debug_color);
                screen.set_pixel(y, 3, self.debug_color);
                screen.set_pixel(y, 4, self.debug_color);
                screen.set_pixel(y, 5, self.debug_color);
                screen.set_pixel(y, 6, self.debug_color);
            }
        }
    }    

    fn update_mode(&mut self, cycles: u32) {
        self.mode_cycles += cycles;
        match self.lcdc_mode {
            LCDCMode::HBLANK => {                
                if self.need_change_mode(HBLANK_CYCLES) {
                    self.render_scan_line();
                    self.increase_line();
                    if self.ly_coord.r() == VBLANK_START_LINE {
                        self.change_mode_and_interrupt(LCDCMode::VBLANK);
                    } else {
                        self.change_mode_and_interrupt(LCDCMode::OAM);
                    }
                    self.mode_cycles -= HBLANK_CYCLES;
                }
            }
            LCDCMode::VBLANK => {
                if self.mode_cycles > CYCLES_PER_LINE {
                    self.increase_line();
                    self.mode_cycles -= CYCLES_PER_LINE;
                }
                if self.ly_coord.r() == VBLANK_END_LINE {
                    self.change_mode_and_interrupt(LCDCMode::OAM);                   
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

    fn render_scan_line(&mut self) {
        self.render_background_line();
    }

    fn render_background_line(&mut self) {
        let background_tile_map_start = 
            if self.lcd_control.is_bit_set(B_BG_TILE_MAP_SELECT) {
                BG_TILE_MAP_2_START
            } else {
                BG_TILE_MAP_1_START
            };
        let background_tile_map = &self.vram.load_chunk(background_tile_map_start, 32 * 32);
        hex_print("Background Tile Map", background_tile_map, 32);
    }
}

impl MemoryRegion for GPU {
    fn read_byte(&self, addr: u16) -> u8 {
        if self.vram.in_region(addr) {
            self.vram.read_byte(addr)
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
        if self.vram.in_region(addr) {
            self.vram.write_byte(addr, val)
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
        self.vram.in_region(addr) 
        || self.vram.in_region(addr)
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