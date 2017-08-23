use hardware::cartridge::Cartridge;

const ROM_START                 : u16 = 0x0000;
const ROM_END                   : u16 = 0x7FFF;

const GRAPHICS_RAM_START        : u16 = 0x8000;
const GRAPHICS_RAM_END          : u16 = 0x9FFF;

const CARTRIDGE_RAM_START       : u16 = 0xA000;
const CARTRIDGE_RAM_END         : u16 = 0xBFFF;

const INTERNAL_RAM_START        : u16 = 0xC000;
const INTERNAL_RAM_END          : u16 = 0xDFFF;

const INTERNAL_RAM_ECHO_START   : u16 = 0xE000;
const INTERNAL_RAM_ECHO_END     : u16 = 0xFDFF;

const SPRITE_INFO_START         : u16 = 0xFE00;
const SPRITE_INFO_END           : u16 = 0xFE9F;

const IO_MEMORY_START           : u16 = 0xFF00;
const IO_MEMORY_END             : u16 = 0xFF7F;

const ZERO_PAGE_RAM_START       : u16 = 0xFF80;
const ZERO_PAGE_RAM_END         : u16 = 0xFFFF;

struct MemoryRegion {
    start: u16,
    end: u16
}

impl MemoryRegion {
    fn new(start: u16, end: u16) -> Self {
        MemoryRegion {
            start: start,
            end: end
        }
    }

    pub fn in_region(&self, addr: u16) -> bool {
        addr >= self.start && addr <= self.end
    }
}


pub struct BUS {
    cartridge : Cartridge,

    // Memory regions
    region_rom: MemoryRegion,
    region_graphics: MemoryRegion,
    region_cartridge_ram: MemoryRegion,
    region_ram: MemoryRegion,
    region_ram_echo: MemoryRegion,
    region_sprites: MemoryRegion,
    region_io: MemoryRegion,
    region_zero_ram: MemoryRegion
}

impl BUS {
    pub fn new(cartridge: Cartridge) -> Self {
        BUS {
            cartridge: cartridge,

            region_rom: MemoryRegion::new(ROM_START, ROM_END),
            region_graphics: MemoryRegion::new(GRAPHICS_RAM_START, GRAPHICS_RAM_END),
            region_cartridge_ram: MemoryRegion::new(CARTRIDGE_RAM_START, CARTRIDGE_RAM_END),
            region_ram: MemoryRegion::new(INTERNAL_RAM_START, INTERNAL_RAM_END),
            region_ram_echo: MemoryRegion::new(INTERNAL_RAM_ECHO_START, INTERNAL_RAM_ECHO_END),
            region_sprites: MemoryRegion::new(SPRITE_INFO_START, SPRITE_INFO_END),
            region_io: MemoryRegion::new(IO_MEMORY_START, IO_MEMORY_END),
            region_zero_ram: MemoryRegion::new(ZERO_PAGE_RAM_START, ZERO_PAGE_RAM_END),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        if self.region_rom.in_region(addr) {
            return self.cartridge.read_byte(addr)
        }
        0xFFFF
    }

    pub fn write_byte(addr: u16, val: u8) {
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        if self.region_rom.in_region(addr) {
            return self.cartridge.read_word(addr)
        }
        0xFFFF
    }

    pub fn write_word(addr: u16, val: u16) {

    }
}