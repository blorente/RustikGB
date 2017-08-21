use std::fmt;
use std::marker;
use cpu::instructions;
use cpu::cartridge;

#[derive(Default)]
struct Register {
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

    pub fn increase_by(&mut self, amount: u16) {self.val += amount;}
 }


pub struct RegBank {
    af : Register,
    bc : Register,
    de : Register,
    hl : Register,
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
    registers : RegBank,
    pub sp : Register,
    pub pc : Register,
}


impl  Default for CPU {
    fn default() -> CPU {
        CPU {
            registers : Default::default(),
            sp : Register::new(0xFFFE),            
            pc : Register::new(0x0100),
        }
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "{}sp: 0x{:<4X}\npc: 0x{:<4X}", 
                self.registers,
                self.sp.value(),
                self.pc.value())
    }
}

impl CPU {
    pub fn run(&mut self, rom_buf: &cartridge::Cartridge) {
        let mut instr_set = instructions::InstructionSet::new();
        loop {
            let opcode = rom_buf.read_byte(self.pc.value());
            println!("Running inst {:X}", opcode);            
            if !instr_set.is_implemented(opcode) {
                println!("Unimplemented instruction 0x{:X}", opcode);
                println!("Processor state:\n{}", self);
                break;
            } else {
                let cycles = instr_set.exec(self, cartridge, opcode);  
                self.pc.increase_by(1); 
            }         
        }
    }
}

