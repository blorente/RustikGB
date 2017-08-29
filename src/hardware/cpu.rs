use std::fmt;
use hardware::instructions;
use hardware::memory::bus;
use hardware::debugger;
use hardware::registers::Register;
use hardware::video::screen::Screen;
use hardware::debugger::Debugger;
use hardware::instructions::InstructionSet;

const CYCLES_PER_FRAME: u32 = 70244;

pub struct RegBank {
    pub a : Register<u8>,
    pub f : Register<u8>,
    pub b : Register<u8>,
    pub c : Register<u8>,
    pub d : Register<u8>,
    pub e : Register<u8>,
    pub h : Register<u8>,
    pub l : Register<u8>,
}

impl fmt::Display for RegBank {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "a: 0x{:02X} | f: 0x{:02X}\nb: 0x{:02X} | c: 0x{:02X}\nd: 0x{:02X} | e: 0x{:02X}\nh: 0x{:02X} | l: 0x{:02X}",
        self.a.r(), self.f.r(),
        self.b.r(), self.c.r(),
        self.d.r(), self.e.r(),
        self.h.r(), self.l.r())
    }
}

impl Default for RegBank {
    fn default() -> Self {
        RegBank {
            a: Register::new(0x00),
            f: Register::new(0x00),
            b: Register::new(0x00),
            c: Register::new(0x00),
            d: Register::new(0x00),
            e: Register::new(0x00),
            h: Register::new(0x00),
            l: Register::new(0x00),   
        }
    }
}

impl RegBank {
    pub fn af(&self) -> u16 {
        (self.a.r() as u16) << 8 | (self.f.r() as u16)
    }

    pub fn bc(&self) -> u16 {
        (self.b.r() as u16) << 8 | (self.c.r() as u16)
    }

    pub fn de(&self) -> u16 {
        (self.d.r() as u16) << 8 | (self.e.r() as u16)
    }

    pub fn hl(&self) -> u16 {
        (self.h.r() as u16) << 8 | (self.l.r() as u16)
    }

    pub fn af_w(&mut self, word: u16) {
        self.a.w(((word & 0xFF00) >> 8) as u8);
        self.f.w((word & 0x00FF) as u8);
    }

    pub fn bc_w(&mut self, word: u16) {
        self.b.w(((word & 0xFF00) >> 8) as u8);
        self.c.w((word & 0x00FF) as u8);
    }

    pub fn de_w(&mut self, word: u16) {
        self.d.w(((word & 0xFF00) >> 8) as u8);
        self.e.w((word & 0x00FF) as u8);
    }

    pub fn hl_w(&mut self, word: u16) {
        self.h.w(((word & 0xFF00) >> 8) as u8);
        self.l.w((word & 0x00FF) as u8);
    }
}

pub struct CPU {
    pub bus: bus::BUS,
    pub regs : RegBank,
    pub sp : Register<u16>,
    pub pc : Register<u16>,

    cycles: u32
}

impl fmt::Display for CPU {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let sp = self.sp.r();
        writeln!(fmt, "{}sp: 0x{:04X} ({:02X}, {:02X}, {:02X})\npc: 0x{:04X}", 
                self.regs,
                self.sp.r(), 
                    self.read_byte(sp),
                    if sp <= 0xFFFE {self.read_byte(sp + 1)} else {0x00},
                    if sp <= 0xFFFD {self.read_byte(sp + 2)} else {0x00},
                self.pc.r())
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
            sp : Register::new(0x0000),            
            pc : Register::new(0x0000),
            cycles: 0
        }
    }

    pub fn run_frame(&mut self, debugger: &mut Debugger, instr_set: &InstructionSet) {
        while self.cycles < CYCLES_PER_FRAME {
            let mut bitwise = false;
            let old_pc = self.pc.r();
            let mut opcode = self.fetch_byte_immediate();
            if opcode == 0xCB {bitwise = true; opcode = self.fetch_byte_immediate();}

            debugger.stop_if_needed(old_pc, self, &instr_set);              
             
            if !instr_set.is_implemented(opcode, bitwise) {
                println!("Unimplemented instruction {}0x{:0X}\nProcessor state:\n{}", 
                        if bitwise {"(CB)"} else {""},
                        opcode,
                        self);
                panic!("Unimplemented instruction!");
            } 

            self.cycles += self.step(&instr_set, opcode, bitwise);
        }

        self.cycles -= CYCLES_PER_FRAME;
    }

    fn step(&mut self, instr_set: &instructions::InstructionSet, opcode: u8, bitwise: bool) -> u32 {
            let step_cycles;
            if !bitwise {
                step_cycles = instr_set.exec(self, opcode) * 4;
            } else {
                step_cycles = instr_set.exec_bit(self, opcode) * 4;
            }            

            self.bus.step(step_cycles);
            step_cycles
    }

    pub fn fetch_byte_immediate(&mut self) -> u8 {
        let res = self.bus.read_byte(self.pc.r());
        let oldpc = self.pc.r();
        self.pc.w(oldpc + 1);
        res
    }

    pub fn fetch_word_immediate(&mut self) -> u16 {
        let res = self.bus.read_word(self.pc.r());
        let oldpc = self.pc.r();
        self.pc.w(oldpc + 2);
        res
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.bus.read_byte(addr)
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.bus.write_byte(addr, val)
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        self.bus.read_word(addr)
    }

    pub fn write_word(&mut self, addr: u16, val: u16) {
        self.bus.write_word(addr, val)
    }

    pub fn is_flag_set(&self, flag: CPUFlags) -> bool {
        self.regs.f.r() & (flag as u8) > 0
    }

    pub fn set_flag(&mut self, flag: CPUFlags, val: bool) {
        if val {
            let new_f = self.regs.f.r() | flag as u8;
            self.regs.f.w(new_f & 0xF0);
        } else {
            let new_f = self.regs.f.r() & !(flag as u8);
            self.regs.f.w(new_f & 0xF0); 
        }
    }

    pub fn disable_interrupts_delayed(&mut self) {
        self.bus.disable_in_next_step();        
    }

    pub fn enable_interrupts_delayed(&mut self) {
        self.bus.enable_in_next_step();
    }

    pub fn push_word(&mut self, val: u16) {
        self.push(((val & 0xFF00) >> 8) as u8);
        self.push((val & 0xFF) as u8)
    }

    pub fn pop_word(&mut self) -> u16 {
        let lo = self.pop() as u16;
        let hi = self.pop() as u16;
        (hi << 8) | lo
    }

    pub fn push(&mut self, val: u8) {
        let new_sp = self.sp.r().wrapping_sub(1);
        self.sp.w(new_sp);
        self.bus.write_byte(new_sp, val);
    }

    pub fn pop(&mut self) -> u8 {
        let sp = self.sp.r();
        let res = self.bus.read_byte(sp);
        self.sp.w(sp.wrapping_add(1));
        res
    }
}

