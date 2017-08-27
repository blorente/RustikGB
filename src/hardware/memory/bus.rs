use hardware::cartridge::Cartridge;
use hardware::memory::ioregs::IORegs;
use hardware::memory::memory_region::MemoryRegion;
use hardware::memory::plain_ram::PLAIN_RAM;
use hardware::video::gpu::GPU;

const BIOS_START                : u16 = 0x0000;
const BIOS_END                  : u16 = 0x00FF;

const CARTRIDGE_RAM_START       : u16 = 0xA000;
const CARTRIDGE_RAM_END         : u16 = 0xBFFF;

const INTERNAL_RAM_START        : u16 = 0xC000;
const INTERNAL_RAM_END          : u16 = 0xDFFF;

const INTERNAL_RAM_ECHO_START   : u16 = 0xE000;
const INTERNAL_RAM_ECHO_END     : u16 = 0xFDFF;

const ZERO_PAGE_RAM_START       : u16 = 0xFF80;
const ZERO_PAGE_RAM_END         : u16 = 0xFFFF;


pub struct BUS {
    cartridge : Cartridge,
    boot_rom: PLAIN_RAM,
    gpu: GPU,
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
            gpu: GPU::new(),
            storage_ram: PLAIN_RAM::new(INTERNAL_RAM_START, INTERNAL_RAM_END),
            storage_zero_ram: PLAIN_RAM::new(ZERO_PAGE_RAM_START, ZERO_PAGE_RAM_END),
            io_registers: IORegs::new(),

            in_bios: true
        }
    }

    pub fn step(&mut self, cycles: u32) {
        self.gpu.step(cycles);
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        if self.in_bios && self.boot_rom.in_region(addr) {
            return self.boot_rom.read_byte(addr);
        } else if self.cartridge.in_region(addr) {
            return self.cartridge.read_byte(addr)
        } else if self.gpu.in_region(addr) {
            return self.gpu.read_byte(addr);
        } else if self.storage_ram.in_region(addr) | (addr >= INTERNAL_RAM_ECHO_START && addr <= INTERNAL_RAM_ECHO_END) {
            return self.storage_ram.read_byte(addr);
        } else if self.storage_zero_ram.in_region(addr) {
            return self.storage_zero_ram.read_byte(addr);
        } else if self.io_registers.in_region(addr) {
            return self.io_registers.read_byte(addr);
        }
        panic!("Trying to read byte from unrecognized address: 0x{:X}", addr);
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {   
        if self.gpu.in_region(addr) {
            self.gpu.write_byte(addr, val);
        } else if self.storage_ram.in_region(addr) | (addr >= INTERNAL_RAM_ECHO_START && addr <= INTERNAL_RAM_ECHO_END) {
            self.storage_ram.write_byte(addr, val);
        } else if self.storage_zero_ram.in_region(addr) {
            self.storage_zero_ram.write_byte(addr, val);
        } else if self.io_registers.in_region(addr) {
            self.io_registers.write_byte(addr, val);
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