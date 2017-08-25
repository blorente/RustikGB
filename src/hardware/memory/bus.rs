use hardware::cartridge::Cartridge;
use hardware::memory::ioregs::IORegs;
use std::fmt;

const BIOS_START                : u16 = 0x0000;
const BIOS_END                  : u16 = 0x00FF;

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

impl fmt::Display for MemoryRegion {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "(0x{:4X}, {:4X})", 
                self.start, self.end)
    }
}

struct RAM {
    pub storage: Vec<u8>
}

impl RAM {
    pub fn new(size: usize) -> Self {
        RAM {
            storage: vec![0x0; size]
        }
    }

    pub fn from_data(data: Box<[u8]>) -> Self {
        RAM {
            storage: data.to_vec()
        }
    }
}


pub struct BUS {
    cartridge : Cartridge,
    boot_rom: RAM,
    graphics_ram: RAM,
    storage_ram: RAM,
    storage_zero_ram: RAM,
    io_registers: IORegs,     

    pub in_bios: bool,

    // Memory regions
    region_bios: MemoryRegion,
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
    pub fn new(boot_rom: Box<[u8]>, cartridge: Cartridge) -> Self {
        BUS {
            cartridge: cartridge,
            boot_rom: RAM::from_data(boot_rom),
            graphics_ram: RAM::new((GRAPHICS_RAM_END - GRAPHICS_RAM_START + 1) as usize),
            storage_ram: RAM::new((INTERNAL_RAM_END - INTERNAL_RAM_START + 1) as usize),
            storage_zero_ram: RAM::new((ZERO_PAGE_RAM_END - ZERO_PAGE_RAM_START + 1) as usize),
            io_registers: IORegs::new(),

            in_bios: true,

            region_bios: MemoryRegion::new(BIOS_START, BIOS_END),
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
        if self.in_bios && self.region_bios.in_region(addr) {
            return self.boot_rom.storage[addr as usize];
        } else if self.region_rom.in_region(addr) {
            return self.cartridge.read_byte(addr)
        } else if self.region_graphics.in_region(addr) {
            let tru_addr = addr - self.region_graphics.start;
            return self.graphics_ram.storage[tru_addr as usize];
        } else if self.region_ram.in_region(addr) | self.region_ram_echo.in_region(addr) {
            let tru_addr = addr - self.region_ram.start;
            return self.storage_ram.storage[tru_addr as usize];
        } else if self.region_zero_ram.in_region(addr) {
            let tru_addr = addr - self.region_zero_ram.start;
            return self.storage_zero_ram.storage[tru_addr as usize];
        } else if self.region_io.in_region(addr) {
            return self.io_registers.read_reg(addr);
        }
        panic!("Trying to read byte from unrecognized address: 0x{:X}", addr);
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {   
        if self.region_graphics.in_region(addr) {
            let tru_addr = addr - self.region_graphics.start;
            self.graphics_ram.storage[tru_addr as usize] = val;
        } else if self.region_ram.in_region(addr) | self.region_ram_echo.in_region(addr) {
            let tru_addr = addr - self.region_ram.start;
            self.storage_ram.storage[tru_addr as usize] = val;
        } else if self.region_zero_ram.in_region(addr) {
            let tru_addr = addr - self.region_zero_ram.start;
            self.storage_zero_ram.storage[tru_addr as usize] = val;
        } else if self.region_io.in_region(addr) {
            return self.io_registers.write_reg(addr, val);
        } else {
            panic!("Trying to write byte 0x{:X} to unrecognized address: 0x{:X}", val, addr);
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr + 1) as u16;
        hi << 8 | lo
    }

    pub fn write_word(&mut self, addr: u16, val: u16) {
        let first = ((val & 0xFF00) >> 8) as u8;
        let second = (val & 0x00FF) as u8;
        self.write_byte(addr, first);
        self.write_byte(addr + 1, second);
    }
}