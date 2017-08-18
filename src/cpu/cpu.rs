use std::fmt;
use std::marker;

#[derive(Default)]
struct Register {
    val: u16,
}

impl Register {
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
        self.af.r_hi(), self.af.r_lo(),
        self.bc.r_hi(), self.bc.r_lo(),
        self.de.r_hi(), self.de.r_lo(),
        self.hl.r_hi(), self.hl.r_lo(),
        self.sp.value(),
        self.pc.value())
    }
}

impl Default for RegBank {
    fn default() -> Self {
        RegBank {
            af: Register {val: 0xABCD},
            bc: Register {val: 0x0000},
            de: Register {val: 0x0000},
            hl: Register {val: 0x0000},
            sp: Register {val: 0x0000},
            pc: Register {val: 0x0000},
        }
    }
}


pub struct Instruction<'i> {
    dissassembly : &'static str,
    operand_num : u8,
    execution : Box<Fn(&mut CPU<'i>, u16) + 'i>,
}

impl<'i> Instruction<'i> {
    pub fn new<F: Fn(&mut CPU<'i>, u16) + 'i> (dissassembly: &'static str, operand_num : u8, func: F) -> Instruction<'i> {
        Instruction {
            dissassembly: dissassembly,
            operand_num: operand_num,
            execution: Box::new(func)
        }
    }
}

pub struct CPU <'i> {
    registers : RegBank,
    instructions : [Instruction<'i>; 256]
}


impl<'i>  Default for CPU<'i> {
    fn default() -> CPU<'i> {
        CPU {
            registers : Default::default(),
            instructions : create_isa(),
        }
    }
}

impl<'i> fmt::Display for CPU<'i> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "{}", self.registers)
    }
}

impl<'i> CPU<'i> {
    pub fn run(&mut self, rom_buf: Box<[u8]>) {
        loop {
            let opcode = rom_buf[self.registers.pc.value() as usize];
            println!("Running inst {:X}", opcode);
            if self.instructions[opcode as usize].dissassembly == "Unimp" {
                println!("Unimplemented instruction 0x{:X}", opcode);
                println!("Processor state:\n{}", self);
                break;
            }   
        }
    }
}

macro_rules! inst {
    ($x:expr, $y:expr, $f:expr) => {{
        let inst = Instruction::new($x, $y, $f);
        inst
    }}    
}
pub fn create_isa <'i>() -> [Instruction<'i>; 256]{
    [
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),         
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x);})
    ]
}

