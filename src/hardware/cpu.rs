use std::fmt;
use hardware::instructions;
use hardware::bus;

#[derive(Default)]
pub struct Register {
    val: u16,
}

impl Register {
    pub fn new(value: u16) -> Self {
        Register {
            val: value
        }
    }

    pub fn value(&self) -> u16 {
        self.val
    }

    pub fn r_hi(&self) -> u8 {
        ((self.val & 0xFF00) >> 8) as u8
    }

    pub fn r_lo(&self) -> u8 {
        (self.val & 0x00FF) as u8
    }

    pub fn w_lo(& mut self, data: u8) {
        self.val = (self.val & 0xFF00) | data as u16;
    }

    pub fn w_hi(& mut self, data: u8) {
        self.val = (self.val & 0x00FF) | ((data as u16) << 8);
    }

    pub fn w_all(&mut self, data: u16) {
        self.val = data;
    }

    pub fn increase_by(&mut self, amount: u16) {self.val += amount;}
 }


pub struct RegBank {
    pub af : Register,
    pub bc : Register,
    pub de : Register,
    pub hl : Register,
}

impl fmt::Display for RegBank {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "a: 0x{:<2X} | f: 0x{:<2X}\nb: 0x{:<2X} | c: 0x{:<2X}\nd: 0x{:<2X} | e: 0x{:<2X}\nh: 0x{:<2X} | l: 0x{:<2X}",
        self.af.r_hi(), self.af.r_lo(),
        self.bc.r_hi(), self.bc.r_lo(),
        self.de.r_hi(), self.de.r_lo(),
        self.hl.r_hi(), self.hl.r_lo())
    }
}

impl Default for RegBank {
    fn default() -> Self {
        RegBank {
            af: Register {val: 0x01B0},
            bc: Register {val: 0x0013},
            de: Register {val: 0x00D8},
            hl: Register {val: 0x014D},
        }
    }
}

pub struct CPU {
    bus: bus::BUS,
    pub regs : RegBank,
    pub sp : Register,
    pub pc : Register,
}

impl fmt::Display for CPU {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "{}sp: 0x{:<4X}\npc: 0x{:<4X}", 
                self.regs,
                self.sp.value(),
                self.pc.value())
    }
}

pub enum CPUFlags {
    Z = 0b10000000,
    N = 0b01000000,
    H = 0b00100000,
    C = 0b00010000
}

impl CPU {
    pub fn new(bus: bus::BUS) -> Self {
        CPU {
            bus: bus,
            regs : Default::default(),
            sp : Register::new(0xFFFE),            
            pc : Register::new(0x0100),
        }
    }

    pub fn run(&mut self) {
        let mut instr_set = instructions::InstructionSet::new();
        loop {            
            let opcode = self.read_byte(self.pc.value());
            println!("PC: {:<4X}, Opcode {:<2X}",
                    self.pc.value(),
                    opcode);      
            if !instr_set.is_implemented(opcode) {
                println!("Unimplemented instruction 0x{:X}", opcode);
                println!("Processor state:\n{}", self);
                break;
            } else {
                let cycles = instr_set.exec(self, opcode);  
                self.pc.increase_by(1); 
            }         
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.bus.read_byte(addr)
    }

    pub fn write_byte(&self, addr: u16, val: u8) {
        self.bus.write_byte(addr, val)
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        self.bus.read_word(addr)
    }

    pub fn write_word(&self, addr: u16, val: u16) {
        self.bus.write_word(addr, val)
    }

    pub fn is_flag_set(&self, flag: CPUFlags) -> bool {
        self.regs.af.r_lo() & (flag as u8) > 0
    }

    pub fn set_flag(&mut self, flag: CPUFlags, val: bool) {
        if (val) {
            self.regs.af.val = self.regs.af.val | (flag as u16)
        } else {
            self.regs.af.val = self.regs.af.val & !(flag as u16)
        }
        self.regs.af.val &= 0xFFF0;
    }
}

