use hardware::cartridge::Cartridge;
use hardware::memory::ioregs::IORegs;
use hardware::memory::memory_region::MemoryRegion;
use hardware::memory::memory_region::BitAccess;

const BIOS_START                : u16 = 0x0000;
const BIOS_END                  : u16 = 0x00FF;

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

const ZERO_PAGE_RAM_START       : u16 = 0xFF80;
const ZERO_PAGE_RAM_END         : u16 = 0xFFFF;

struct PLAIN_RAM {
    pub storage: Vec<u8>,
    start: u16,
    end: u16
}

impl PLAIN_RAM {
    pub fn new(start: u16, end: u16) -> Self {
        PLAIN_RAM {
            start: start,
            end: end,
            storage: vec![0x0; end as usize - start as usize + 1]
        }
    }

    pub fn from_data(start: u16, end: u16, data: Box<[u8]>) -> Self {
        PLAIN_RAM {
            start: start,
            end: end,
            storage: data.to_vec()
        }
    }
}

impl MemoryRegion for PLAIN_RAM {
    fn read_byte(&self, addr: u16) -> u8 {
        self.storage[addr as usize]
    }


    fn write_byte(&mut self, addr: u16, val: u8) {
        self.storage[addr as usize] = val;
    }

    fn in_region(&self, addr: u16) -> bool {
        addr >= self.start() && addr <= self.end()
    }
    fn start(&self) -> u16 {
        self.start
    }
    fn end(&self) -> u16 {
        self.end
    }
}

impl BitAccess for PLAIN_RAM {    
    fn read_bit(&self, addr: u16, bit: u8) -> bool {
        let val = self.read_byte(addr);
        val & (1 << bit) > 0
    }

    fn set_bit(&mut self, addr: u16, bit: u8, val: bool) {
        let cur_val = self.read_byte(addr);
        if val {
            self.storage[addr as usize] = cur_val | (1 << bit);
        } else {
            self.storage[addr as usize] = cur_val & !(1 << bit);
        }
    }
}


pub struct BUS {
    cartridge : Cartridge,
    boot_rom: PLAIN_RAM,
    graphics_ram: PLAIN_RAM,
    storage_ram: PLAIN_RAM,
    storage_zero_ram: PLAIN_RAM,
    io_registers: IORegs,     

    pub in_bios: bool
}

impl BUS {
    pub fn new(boot_rom: Box<[u8]>, cartridge: Cartridge) -> Self {
        BUS {
            cartridge: cartridge,
            boot_rom: PLAIN_RAM::from_data(BIOS_START, BIOS_END, boot_rom),
            graphics_ram: PLAIN_RAM::new(GRAPHICS_RAM_START, GRAPHICS_RAM_END),
            storage_ram: PLAIN_RAM::new(INTERNAL_RAM_START, INTERNAL_RAM_END),
            storage_zero_ram: PLAIN_RAM::new(ZERO_PAGE_RAM_START, ZERO_PAGE_RAM_END),
            io_registers: IORegs::new(),

            in_bios: true
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        if self.in_bios && self.boot_rom.in_region(addr) {
            return self.boot_rom.storage[addr as usize];
        } else if self.cartridge.in_region(addr) {
            return self.cartridge.read_byte(addr)
        } else if self.graphics_ram.in_region(addr) {
            let tru_addr = addr - self.graphics_ram.start;
            return self.graphics_ram.storage[tru_addr as usize];
        } else if self.storage_ram.in_region(addr) | (addr >= INTERNAL_RAM_ECHO_START && addr <= INTERNAL_RAM_ECHO_END) {
            let tru_addr = addr - self.storage_ram.start;
            return self.storage_ram.storage[tru_addr as usize];
        } else if self.storage_zero_ram.in_region(addr) {
            let tru_addr = addr - self.storage_zero_ram.start;
            return self.storage_zero_ram.storage[tru_addr as usize];
        } else if self.io_registers.in_region(addr) {
            return self.io_registers.read_byte(addr);
        }
        panic!("Trying to read byte from unrecognized address: 0x{:X}", addr);
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {   
        if self.graphics_ram.in_region(addr) {
            let tru_addr = addr - self.graphics_ram.start;
            self.graphics_ram.storage[tru_addr as usize] = val;
        } else if self.storage_ram.in_region(addr) | (addr >= INTERNAL_RAM_ECHO_START && addr <= INTERNAL_RAM_ECHO_END) {
            let tru_addr = addr - self.storage_ram.start;
            self.storage_ram.storage[tru_addr as usize] = val;
        } else if self.storage_zero_ram.in_region(addr) {
            let tru_addr = addr - self.storage_zero_ram.start;
            self.storage_zero_ram.storage[tru_addr as usize] = val;
        } else if self.io_registers.in_region(addr) {
            return self.io_registers.write_byte(addr, val);
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