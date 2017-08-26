use hardware::registers::Register;
use hardware::memory::memory_region::MemoryRegion;
use hardware::memory::memory_region::BitAccess;

const IO_MEMORY_START           : u16 = 0xFF00;
const IO_MEMORY_END             : u16 = 0xFF7F;

pub struct IORegs {
    contents: [Register<u8>; 0x4C]
}

impl MemoryRegion for IORegs {
    fn read_byte(&self, addr: u16) -> u8 {
        self.contents[(addr - 0xFF00) as usize].r()
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        self.contents[(addr - 0xFF00) as usize].w(val);
    }

    fn in_region(&self, addr: u16) -> bool {
        addr >= self.start() && addr <= self.end()
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
            contents: [Register::new(0); 0x4C]
        }
    }
}

impl BitAccess for IORegs {
    fn read_bit(&self, addr: u16, bit: u8) -> bool {
        self.contents[(addr - 0xFF00) as usize].is_bit_set(bit)
    }

    fn set_bit(&mut self, addr: u16, bit: u8, val: bool) {
        self.contents[(addr - 0xFF00) as usize].set_bit(bit, val);
    }
}

