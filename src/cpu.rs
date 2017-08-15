pub mod cpu {
use std::fmt;


#[derive(Default)]
struct Register {
    hi : u8,
    lo : u8,
}

impl Register {
    pub fn value(&self) -> u16 {
        let ret : u16 = ((self.hi as u16) << 8) | self.lo as u16;
        return ret;
     }
}


pub struct RegBank {
    af : Register,
    bc : Register,
    de : Register,
    hl : Register,
    sp : Register,
    pc : Register,
}

impl fmt::Display for RegBank {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "a: 0x{:<2x} | f: 0x{:<2x}\nb: 0x{:<2x} | c: 0x{:<2x}\nd: 0x{:<2x} | e: 0x{:<2x}\nh: 0x{:<2x} | l: 0x{:<2x}\nsp: 0x{:<4x}\npc: 0x{:<4x}",
        self.af.hi, self.af.lo,
        self.bc.hi, self.bc.lo,
        self.de.hi, self.de.lo,
        self.hl.hi, self.hl.lo,
        self.sp.value(),
        self.pc.value())
    }
}

impl Default for RegBank {
    fn default() -> RegBank {
        RegBank {
            af: Register {hi: 0xAB, lo: 0xCD},
            bc: Register {hi: 0x1, lo: 0},
            de: Register {hi: 0, lo: 0},
            hl: Register {hi: 0, lo: 0},
            sp: Register {hi: 0, lo: 0},
            pc: Register {hi: 0, lo: 0},
        }
    }
}

pub struct Instruction<'i> {
    dissassembly : &'static str,
    operand_num : u8,
    execution : Box<Fn(u16) + 'i>,
}

impl<'i> Instruction<'i> {
    pub fn new<F: Fn(u16) + 'i> (dissassembly: &'static str, operand_num : u8, func: F) -> Instruction<'i> {
        Instruction {
            dissassembly,
            operand_num,
            execution: Box::new(func)
        }
    }
}

pub struct CPU <'i> {
    registers : RegBank,
    instructions : [Instruction<'i>; 1]
}


impl<'i>  Default for CPU<'i> {
    fn default() -> CPU<'i> {
        CPU {
            registers : Default::default(),
            instructions : [
                Instruction::new("NOP", 0, |x|{println!("{}", x)})
            ],
        }
    }
}

impl<'i> fmt::Display for CPU<'i> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "{}", self.registers)
    }
}
}
