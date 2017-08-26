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
const Y_COORD_ADDR                  : u16 = 0xFF44;
const LY_COMPLARE_ADDR              : u16 = 0xFF45;
const DMA_START_ADDR                : u16 = 0xFF46;
const BG_PALLETE_ADDR               : u16 = 0xFF47;
const OBJECT_PALETTE_ADDR           : u16 = 0xFF49;
const WINDOW_Y_ADDR                 : u16 = 0xFF4A;
const WINDOW_X_ADDR                 : u16 = 0xFF4B;

pub struct GPU {
    vram: PLAIN_RAM,
    sprite_oam: PLAIN_RAM,

    // IO Registers for the GPU
    lcd_control:    Register<u8>,  
    lcd_status:     Register<u8>,   
    scroll_y:       Register<u8>,
    scroll_x:       Register<u8>,
    y_coord_:       Register<u8>,
    ly_compare:     Register<u8>,
    dma_start:      Register<u8>,
    bg_palette:     Register<u8>,
    obj_palette:    Register<u8>,
    window_y:       Register<u8>,
    window_x:       Register<u8>,
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            vram: PLAIN_RAM::new(VRAM_START, VRAM_END),
            sprite_oam: PLAIN_RAM::new(SPRITE_OAM_START, SPRITE_OAM_END),

            lcd_control:    Register::new(0x00),  
            lcd_status:     Register::new(0x00),   
            scroll_y:       Register::new(0x00),
            scroll_x:       Register::new(0x00),
            y_coord_:       Register::new(0x00),
            ly_compare:     Register::new(0x00),
            dma_start:      Register::new(0x00),
            bg_palette:     Register::new(0x00),
            obj_palette:    Register::new(0x00),
            window_y:       Register::new(0x00),
            window_x:       Register::new(0x00)
        }
    }

    pub fn step(&mut self, cycles: u32) {
        
    }
}

impl MemoryRegion for GPU {
    fn read_byte(&self, addr: u16) -> u8 {
        if self.vram.in_region(addr) {
            self.vram.read_byte(addr)
        } else if self.sprite_oam.in_region(addr) {
            self.sprite_oam.read_byte(addr)
        } else {
            panic!("GPU Can't access memory location {:04X}", addr);
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        if self.vram.in_region(addr) {
            self.vram.write_byte(addr, val)
        } else if self.sprite_oam.in_region(addr) {
            self.sprite_oam.write_byte(addr, val)
        } else {
            panic!("GPU Can't access memory location {:04X}", addr);
        }
    }

    fn in_region(&self, addr: u16) -> bool {
        self.vram.in_region(addr) 
        || self.vram.in_region(addr)
    }

    fn start(&self) -> u16 {
        panic!("GPU Doesn't have a real 'start()'");
    }

    fn end(&self) -> u16 {
        panic!("GPU Doesn't have a real 'end()'");
    }
}