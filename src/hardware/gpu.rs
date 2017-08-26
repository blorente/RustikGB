use hardware::memory::plain_ram::PLAIN_RAM;
use hardware::memory::memory_region::MemoryRegion;

const SPRITE_OAM_START              : u16 = 0xFE00;
const SPRITE_OAM_END                : u16 = 0xFE9F;

const VRAM_START                    : u16 = 0x8000;
// Use in case we need more granularity in the VRAM
const SPRITE_PATTERN_TABLE_START    : u16 = 0x8000;
const SPRITE_PATTERN_TABLE_END      : u16 = 0x8FFF;

const PROBABLY_TILE_TABLE_START     : u16 = 0x9000;
const PROBABLY_TILE_TABLE_END       : u16 = 0x9FFF;

const VRAM_END                      : u16 = 0x9FFF;

pub struct GPU {
    vram: PLAIN_RAM,
    sprite_oam: PLAIN_RAM
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            vram: PLAIN_RAM::new(VRAM_START, VRAM_END),
            sprite_oam: PLAIN_RAM::new(SPRITE_OAM_START, SPRITE_OAM_END),
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