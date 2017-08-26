use hardware::memory::plain_ram::PLAIN_RAM;
use hardware::memory::memory_region::MemoryRegion;
use hardware::memory::memory_region::BitAccess;
use hardware::registers::Register;

const SPRITE_OAM_START              : u16 = 0xFE00;
const SPRITE_OAM_END                : u16 = 0xFE9F;

const VRAM_START                    : u16 = 0x8000;
// Use in case we need more granularity in the VRAM
const SPRITE_PATTERN_TABLE_START    : u16 = 0x8000;
const SPRITE_PATTERN_TABLE_END      : u16 = 0x8FFF;
const PROBABLY_TILE_TABLE_START     : u16 = 0x9000;
const PROBABLY_TILE_TABLE_END       : u16 = 0x9FFF;
const VRAM_END                      : u16 = 0x9FFF;

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

const CYCLES_PER_LINE               : u32 = 456;

pub struct GPU {
    vram: PLAIN_RAM,
    sprite_oam: PLAIN_RAM,

    elapsed_cycles: u32,

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
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            vram: PLAIN_RAM::new(VRAM_START, VRAM_END),
            sprite_oam: PLAIN_RAM::new(SPRITE_OAM_START, SPRITE_OAM_END),

            elapsed_cycles: 0x0,

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
            window_x:       Register::new(0x00)
        }
    }

    pub fn step(&mut self, cycles: u32) {
        self.elapsed_cycles += cycles;
        if self.elapsed_cycles >= CYCLES_PER_LINE {

            let line = (self.ly_coord.r() + 1) % 154;
            self.ly_coord.w(line);

            self.elapsed_cycles -= CYCLES_PER_LINE;
        }
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