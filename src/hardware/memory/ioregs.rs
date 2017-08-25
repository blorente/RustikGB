use hardware::registers::Register;

pub struct IORegs {
    contents: [Register<u8>; 0x4C]
}

impl IORegs {
    pub fn new() -> Self {
        IORegs {
            contents: [Register::new(0); 0x4C]
        }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        self.contents[(addr - 0xFF00) as usize].r()
    }

    pub fn write_reg(&mut self, addr: u16, val: u8) {
        self.contents[(addr - 0xFF00) as usize].w(val);
    }

    pub fn is_bit_set(&self, addr: u16, bit: u8) -> bool {
        self.contents[(addr - 0xFF00) as usize].is_bit_set(bit)
    }

    pub fn set_bit(&mut self, addr: u16, bit: u8, val: bool) {
        self.contents[(addr - 0xFF00) as usize].set_bit(bit, val);
    }
}

