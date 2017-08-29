use hardware::cartridge::Cartridge;
use hardware::memory::ioregs::IORegs;
use hardware::memory::memory_region::MemoryRegion;
use hardware::memory::plain_ram::PLAIN_RAM;
use hardware::video::gpu::GPU;
use hardware::registers::Register;
use hardware::video::screen::Screen;
use hardware::interrupts::Interrupts;
use hardware::interrupts::InterruptType;
use hardware::joypad::Joypad;

use piston_window::*;

const BIOS_START                : u16 = 0x0000;
const BIOS_END                  : u16 = 0x00FF;

const CARTRIDGE_RAM_START       : u16 = 0xA000;
const CARTRIDGE_RAM_END         : u16 = 0xBFFF;

const INTERNAL_RAM_START        : u16 = 0xC000;
const INTERNAL_RAM_END          : u16 = 0xDFFF;

const INTERNAL_RAM_ECHO_START   : u16 = 0xE000;
const INTERNAL_RAM_ECHO_END     : u16 = 0xFDFF;

const UNUSED_MEMORY_LOW_START   : u16 = 0xFEA0;
const UNUSED_MEMORY_LOW_END     : u16 = 0xFEFF;

const DMA_START_ADDR            : u16 = 0xFF46;

const UNUSED_MEMORY_IO_START    : u16 = 0xFF4C;
const UNUSED_MEMORY_IO_END      : u16 = 0xFF80;

const ZERO_PAGE_RAM_START       : u16 = 0xFF80;
const ZERO_PAGE_RAM_END         : u16 = 0xFFFF;

pub struct BUS {
    cartridge : Cartridge,
    boot_rom: PLAIN_RAM,
    pub gpu: GPU,
    storage_ram: PLAIN_RAM,
    storage_zero_ram: PLAIN_RAM,
    unused_memory: UnusedMemory,
    dma_start: Register<u8>,
    pub interrupt_handler: Interrupts,
    pub joypad: Joypad,

    pub screen: Screen,    
    io_registers: IORegs,     
}

impl BUS {
    pub fn new(window: &mut PistonWindow, boot_rom: Box<[u8]>, cartridge: Cartridge) -> Self {
        BUS {
            cartridge: cartridge,
            boot_rom: PLAIN_RAM::from_data(BIOS_START, BIOS_END, boot_rom),
            gpu: GPU::new(),
            storage_ram: PLAIN_RAM::new(INTERNAL_RAM_START, INTERNAL_RAM_END),
            storage_zero_ram: PLAIN_RAM::new(ZERO_PAGE_RAM_START, ZERO_PAGE_RAM_END),
            unused_memory: UnusedMemory::new(vec![
                (UNUSED_MEMORY_LOW_START, UNUSED_MEMORY_LOW_END),
                (UNUSED_MEMORY_IO_START, UNUSED_MEMORY_IO_END)
                ]),
            interrupt_handler: Interrupts::new(),
            joypad: Joypad::new(),
            dma_start: Register::new(0x00),

            io_registers: IORegs::new(),
            screen: Screen::new(window),
        }
    }

    pub fn step(&mut self, cycles: u32) {
        self.gpu.step(cycles, &mut self.screen, &mut self.interrupt_handler);
        self.joypad.step(cycles, &mut self.interrupt_handler);
        self.interrupt_handler.step(cycles);
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        if self.io_registers.boot_rom_enabled() && self.boot_rom.in_region(addr) {
            return self.boot_rom.read_byte(addr);
        } else if self.cartridge.in_region(addr) {
            return self.cartridge.read_byte(addr)
        } else if self.gpu.in_region(addr) {
            return self.gpu.read_byte(addr);
        } else if self.storage_ram.in_region(addr) | (addr >= INTERNAL_RAM_ECHO_START && addr <= INTERNAL_RAM_ECHO_END) {
            return self.storage_ram.read_byte(addr);
        } else if self.storage_zero_ram.in_region(addr) {
            return self.storage_zero_ram.read_byte(addr);
        } else if self.interrupt_handler.in_region(addr) {
            return self.interrupt_handler.read_byte(addr);
        } else if self.joypad.in_region(addr) {
            return self.joypad.read_byte(addr);
        } else if addr == DMA_START_ADDR {
            panic!("DMA is write only");
        } else if self.io_registers.in_region(addr) {
            return self.io_registers.read_byte(addr);
        } else if self.unused_memory.in_region(addr) {
            return self.unused_memory.read_byte(addr)
        }
        panic!("Trying to read byte from unrecognized address: 0x{:X}", addr);
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {   
        if self.cartridge.in_region(addr) {
            return self.cartridge.write_byte(addr, val);
        } else if self.gpu.in_region(addr) {
            self.gpu.write_byte(addr, val);
        } else if self.storage_ram.in_region(addr) | (addr >= INTERNAL_RAM_ECHO_START && addr <= INTERNAL_RAM_ECHO_END) {
            self.storage_ram.write_byte(addr, val);
        } else if self.interrupt_handler.in_region(addr) {
            self.interrupt_handler.write_byte(addr, val);
        } else if self.storage_zero_ram.in_region(addr) {
            self.storage_zero_ram.write_byte(addr, val);
        } else if self.joypad.in_region(addr) {
            self.joypad.write_byte(addr, val);
        } else if addr == DMA_START_ADDR {
            panic!("DMA Transfer disabled until it's implemented");
        } else if self.io_registers.in_region(addr) {
            self.io_registers.write_byte(addr, val);
        } else if self.unused_memory.in_region(addr) {
            return self.unused_memory.write_byte(addr, val)
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

    pub fn disable_in_next_step(&mut self) {
        self.interrupt_handler.disable_in_next_step();
    }

    pub fn enable_in_next_step(&mut self) {
        self.interrupt_handler.enable_in_next_step();
    }
}

struct UnusedMemory {
    unused_regions: Vec<(u16, u16)>
}

impl UnusedMemory {
    fn new(regions: Vec<(u16, u16)>) -> Self {
        UnusedMemory {
            unused_regions: regions
        }
    }
}

impl MemoryRegion for UnusedMemory {
    fn read_byte(&self, addr: u16) -> u8 {
        0xFF
    }
    fn write_byte(&mut self, addr: u16, val: u8){
        // Writing to unused memory has no effect
    }

    fn in_region(&self, addr: u16) -> bool{
        let mut in_region = false;
        for &(start, end) in &self.unused_regions {
            if addr >= start && addr <= end {
                return true;
            }
        }
        false
    }

    fn start(&self) -> u16{
        panic!("Unused Memory doesn't have just one start")
    }
    fn end(&self) -> u16{
        panic!("Unused Memory doesn't have just one end")
    }
}
