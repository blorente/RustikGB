use hardware::registers::Register;
use hardware::memory::memory_region::MemoryRegion;
use hardware::memory::memory_region::BitAccess;

const IO_MEMORY_START           : u16 = 0xFF00;
const IO_MEMORY_END             : u16 = 0xFF3F;
const BOOT_ROM_ENABLE           : u16 = 0xFF50;

pub struct IORegs {
    contents: [Register<u8>; 0x3F],
    boot_rom_enable: Register<u8>
}

impl MemoryRegion for IORegs {
    fn read_byte(&self, addr: u16) -> u8 {
        quick_fix!({
        if addr == BOOT_ROM_ENABLE {
            self.boot_rom_enable.r()
        } else {
            let tru_addr = addr - self.start();
            self.contents[tru_addr as usize].r()
        }
        }, "IORegs should be gone at some point")
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        quick_fix!({
        if addr == BOOT_ROM_ENABLE {
            self.boot_rom_enable.w(val)
        } else {
            let tru_addr = addr - self.start();
            self.contents[tru_addr as usize].w(val)
        }
        }, "IORegs should be gone at some point");
    }

    fn in_region(&self, addr: u16) -> bool {
        (addr >= self.start() && addr <= self.end()) || addr == BOOT_ROM_ENABLE
    }

    fn start(&self) -> u16 {
        IO_MEMORY_START
    }
    fn end(&self) -> u16 {
        IO_MEMORY_END
    }
}

impl IORegs {
    pub fn new() -> Self {
        IORegs {
            contents: [Register::new(0); 0x3F],
            boot_rom_enable: Register::new(0)
        }
    }

    pub fn boot_rom_enabled(&self) -> bool {
        self.boot_rom_enable.r() == 0
    }
}

impl BitAccess for IORegs {
    fn read_bit(&self, addr: u16, bit: u8) -> bool {
        let tru_addr = addr - self.start();
        self.contents[tru_addr as usize].is_bit_set(bit)
    }

    fn set_bit(&mut self, addr: u16, bit: u8, val: bool) {
        let tru_addr = addr - self.start();
        self.contents[tru_addr as usize].set_bit(bit, val);
    }
}

