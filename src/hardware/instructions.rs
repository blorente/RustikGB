use hardware::cpu::CPU;
use hardware::cpu::CPUFlags;

struct Instruction<'i> {
    pub dissassembly : &'static str,
    op : Box<Fn(&mut CPU, u8) -> u32 + 'i>,
}

impl<'i> Instruction<'i> {
    pub fn new<F: Fn(&mut CPU, u8) -> u32 + 'i> (dissassembly: &'static str, func: F) -> Instruction<'i> {
        Instruction {
            dissassembly: dissassembly,
            op: Box::new(func)
        }
    }

    pub fn execute(&self, cpu: &mut CPU, opcode: u8) -> u32 {
        let closure = &self.op;
        closure(cpu, opcode)
    }
}

pub struct InstructionSet<'i> {
    normal_instructions: Vec<Instruction<'i>>,
    bitwise_instructions: Vec<Instruction<'i>>
}

impl<'i> InstructionSet<'i> {
    pub fn new() -> Self {
        InstructionSet {
            normal_instructions: create_isa(),
            bitwise_instructions: create_bitwise_isa()
        }
    }

    pub fn is_implemented(&self, opcode: u8, bitwise: bool) -> bool {
        match bitwise {
            false => {self.normal_instructions[opcode as usize].dissassembly != "Unimp"}
            true => {self.bitwise_instructions[opcode as usize].dissassembly != "Unimp"}
        }        
    }

    pub fn exec(&self, cpu: &mut CPU, opcode: u8) -> u32 {
        self.normal_instructions[opcode as usize].execute(cpu, opcode)
    }

    pub fn exec_bit(&self, cpu: &mut CPU, opcode: u8) -> u32 {
        self.bitwise_instructions[opcode as usize].execute(cpu, opcode)
    }

    pub fn print_instr(&self, opcode: u8, bitwise: bool) -> &'static str {
        match bitwise {
            true => {self.bitwise_instructions[opcode as usize].dissassembly}
            false => {self.normal_instructions[opcode as usize].dissassembly}
        }
    }
}

macro_rules! inst {
    ($x:expr, $f:expr) => {{
        #[allow(dead_code)]
        let inst = Instruction::new($x, $f);
        inst
    }}    
}

macro_rules! pushall {
    ( $( [$opcode: expr, $x:expr] ),* ) => {
        {
            let mut temp_vec : Vec<Instruction<'i>> = (0..256).map(|x|{inst!("Unimp", |cpu, x|{1})}).collect();
            $(
                temp_vec[$opcode] = $x;
            )*
            temp_vec
        }
    };
}

fn add_carry(other : u8, cpu : &mut CPU) {
    let carry = if cpu.is_flag_set(CPUFlags::C) {1} else {0};
    let a : u8 = cpu.regs.a.r();
    let res : u16 = (a as u16).wrapping_add(other as u16).wrapping_add(carry as u16);
    let res_trunc : u8 = (res & 0xF) as u8;
    cpu.set_flag(CPUFlags::Z, res_trunc == 0);
    cpu.set_flag(CPUFlags::N, false);
    cpu.set_flag(CPUFlags::H, (a & 0xF) + (other & 0xF) + carry > 0xF);
    cpu.set_flag(CPUFlags::C, res > 0xFF);
    cpu.regs.a.w(res_trunc);
}

fn xor(other : u8, cpu : &mut CPU) {
    let a : u8 = cpu.regs.a.r();
    let res : u8 = a ^ other;
    cpu.set_flag(CPUFlags::Z, res == 0);
    cpu.set_flag(CPUFlags::N, false);
    cpu.set_flag(CPUFlags::H, false);
    cpu.set_flag(CPUFlags::C, false);
    cpu.regs.a.w(res);
}

fn or(other : u8, cpu : &mut CPU) {
    let a : u8 = cpu.regs.a.r();
    let res : u8 = a | other;
    cpu.set_flag(CPUFlags::Z, res == 0);
    cpu.set_flag(CPUFlags::N, false);
    cpu.set_flag(CPUFlags::H, false);
    cpu.set_flag(CPUFlags::C, false);
    cpu.regs.a.w(res);
}

fn and(other : u8, cpu : &mut CPU) {
    let a : u8 = cpu.regs.a.r();
    let res : u8 = a & other;
    cpu.set_flag(CPUFlags::Z, res == 0);
    cpu.set_flag(CPUFlags::N, false);
    cpu.set_flag(CPUFlags::H, true);
    cpu.set_flag(CPUFlags::C, false);
    cpu.regs.a.w(res);
}

macro_rules! load_word_imm_u8 {
    ($target_hi_reg: expr, $target_lo_reg: expr, $cpu: expr) => {
        let word = $cpu.fetch_word_immediate();
        $target_hi_reg.w(((word & 0xFF00) >> 8) as u8);
        $target_lo_reg.w((word & 0xFF) as u8);
    };
}

macro_rules! load_word_imm_u16 {
    ($target_reg: expr, $cpu: expr) => {
        let word = $cpu.fetch_word_immediate();
        $target_reg.w(word);
    };
}

macro_rules! load_byte_imm_u8 {
    ($target_reg: expr, $cpu: expr) => {
        let val = $cpu.fetch_byte_immediate();
        $target_reg.w(val);
    };
}

fn store_into_hl(val: u8, cpu: &mut CPU) {
    let hl = cpu.regs.hl();
    cpu.write_byte(hl, val);
}

fn store_hl_into_a(cpu: &mut CPU) {
    let hl = cpu.regs.hl();
    let val = cpu.read_byte(hl);
    cpu.regs.a.w(val);
}

macro_rules! dec {
    ($target_reg: expr, $cpu: expr, $indirect: expr) => {
        let val;
        let addr = $cpu.regs.hl();
        match $indirect {
            true => {val = $cpu.read_byte(addr);}
            false => {val = $target_reg.r()}
        }
        let res = val.wrapping_sub(1);
        match $indirect {
            true => {$cpu.write_byte(addr, res)}
            false => {$target_reg.w(res);}
        }       
        $cpu.set_flag(CPUFlags::Z, res == 0);
        $cpu.set_flag(CPUFlags::N, true);
        $cpu.set_flag(CPUFlags::H, (val & 0x0F) == 0);
    };
}

macro_rules! dec_16 {
    ($target_reg: expr, $cpu: expr) => {
        match $target_reg {
            "BC" => {
                let val = $cpu.regs.bc().wrapping_sub(1);
                $cpu.regs.bc_w(val);
            }
            "DE" => {
                let val = $cpu.regs.de().wrapping_sub(1);
                $cpu.regs.de_w(val);
            }
            "HL" => {
                let val = $cpu.regs.hl().wrapping_sub(1);
                $cpu.regs.hl_w(val);
            }
            "SP" => {
                let val = $cpu.sp.r().wrapping_sub(1);
                $cpu.sp.w(val);
            }
            _ => {panic!("Not a valid target reg for dec_16!");}
        };
    };
}

macro_rules! inc_16 {
    ($target_reg: expr, $cpu: expr) => {
        match $target_reg {
            "BC" => {
                let val = $cpu.regs.bc().wrapping_add(1);
                $cpu.regs.bc_w(val);
            }
            "DE" => {
                let val = $cpu.regs.de().wrapping_add(1);
                $cpu.regs.de_w(val);
            }
            "HL" => {
                let val = $cpu.regs.hl().wrapping_add(1);
                $cpu.regs.hl_w(val);
            }
            "SP" => {
                let val = $cpu.sp.r().wrapping_add(1);
                $cpu.sp.w(val);
            }
            _ => {panic!("Not a valid target reg for inc_16!");}
        }
    };
}

enum JumpImmCond {NZ, Z, NC, C, None}
enum JumpImmMode {IntOffset, Immediate}
fn jump_cond_imm(cpu: &mut CPU, cond: JumpImmCond, mode: JumpImmMode) -> bool {
    let target_addr : u16 = 
        match mode {
            JumpImmMode::IntOffset => {
                let offset = cpu.fetch_byte_immediate() as i8;
                (cpu.pc.r() as u32 as i32 + offset as i32) as u16
            }
            JumpImmMode::Immediate => {
                cpu.fetch_word_immediate()
            }
        };
    let old_pc = cpu.pc.r();

    let new_pc: u16 =  match cond {
        JumpImmCond::NZ => {if !cpu.is_flag_set(CPUFlags::Z) {target_addr} else {cpu.pc.r()} }
        JumpImmCond::Z => {if cpu.is_flag_set(CPUFlags::Z) {target_addr} else {cpu.pc.r()} }
        JumpImmCond::NC => {if !cpu.is_flag_set(CPUFlags::C) {target_addr} else {cpu.pc.r()} }
        JumpImmCond::C => {if cpu.is_flag_set(CPUFlags::C) {target_addr} else {cpu.pc.r()} }
        _ => {target_addr}
    };
    cpu.pc.w(new_pc);
    new_pc != old_pc
}

fn call_cond(cpu: &mut CPU, cond: JumpImmCond) {
    let old_pc = cpu.pc.r();
    if jump_cond_imm(cpu, cond, JumpImmMode::Immediate) {
        let next_inst = old_pc.wrapping_add(2);
        cpu.push_word(next_inst);
    }
}

fn reset(opcode: u8, cpu: &mut CPU) {
    let cur_addr = cpu.pc.r();
    cpu.push_word(cur_addr);
    let addr = (opcode - 0xC7) as u16;
    cpu.pc.w(addr);
}

fn ldh(cpu: &mut CPU, offset: u8, store_into_a: bool) {
    let addr : u16 = 0xFF00u16.wrapping_add(offset as u16);
    match store_into_a {
        true => {let val = cpu.read_byte(addr); cpu.regs.a.w(val);}
        false => {let val = cpu.regs.a.r(); cpu.write_byte(addr, val);}
    }
}


macro_rules! inc {
    ($target_reg: expr, $cpu: expr, $indirect: expr) => {
        let val;
        let addr = $cpu.regs.hl();
        match $indirect {
            true => {val = $cpu.read_byte(addr);}
            false => {val = $target_reg.r()}
        }
        let res = val.wrapping_add(1);
        match $indirect {
            true => {$cpu.write_byte(addr, res)}
            false => {$target_reg.w(res);}
        }       
        $cpu.set_flag(CPUFlags::Z, res == 0);
        $cpu.set_flag(CPUFlags::N, false);
        $cpu.set_flag(CPUFlags::H, (val & 0x0F) + 1 > 0x0F);
    };
}


fn rotate_left_carry(original: u8, cpu: &mut CPU) -> u8 {
    let carry = if cpu.is_flag_set(CPUFlags::C) {1} else {0};
    let rotated = ((original as u16) << 1).wrapping_add(carry);
    cpu.set_flag(CPUFlags::N, false);
    cpu.set_flag(CPUFlags::H, false);
    cpu.set_flag(CPUFlags::C, (original & 0b10000000) > 0);
    (rotated & 0xFF) as u8
}

macro_rules! rotate_left {
    ($target_reg: expr, $cpu: expr) => {
        let original = $target_reg.r();
        let rotated = rotate_left_carry(original, $cpu);
        $target_reg.w(rotated);
    };
}

fn rotate_left_ind(addr: u16, cpu: &mut CPU) {
    let original = cpu.read_byte(addr);
    let rotated = rotate_left_carry(original, cpu);
    cpu.write_byte(addr, rotated);
}

macro_rules! complement {
    ($target_reg: expr, $cpu: expr) => {
        let val = $target_reg.r();
        $target_reg.w(val ^ 0xFF);
        $cpu.set_flag(CPUFlags::N, true);
        $cpu.set_flag(CPUFlags::H, true);
    };
}

macro_rules! pop_into {
    ($reg_hi: expr, $reg_lo: expr, $cpu: expr) => {
        let lo = $cpu.pop();
        let hi = $cpu.pop();
        $reg_lo.w(lo);
        $reg_hi.w(hi);
    };
}

fn jump(addr: u16, cpu: &mut CPU) {
    cpu.pc.w(addr);
}

fn compare_with_a(val: u8, cpu: &mut CPU) {
    let a = cpu.regs.a.r();
    cpu.set_flag(CPUFlags::Z, val == a);
    cpu.set_flag(CPUFlags::N, true);
    cpu.set_flag(CPUFlags::H, (a & 0x0F) < (val & 0x0F));
    cpu.set_flag(CPUFlags::C, a < val);
}

fn sub_to_a(val: u8, cpu: &mut CPU) {
    let a = cpu.regs.a.r();
    let r = a.wrapping_sub(val);
    cpu.regs.a.w(r);
    cpu.set_flag(CPUFlags::Z, r == 0);
    cpu.set_flag(CPUFlags::N, true);
    cpu.set_flag(CPUFlags::H, (a & 0x0F) < (val & 0x0F));
    cpu.set_flag(CPUFlags::C, a < val);
}

fn add_to_a(val: u8, cpu: &mut CPU) {
    let a = cpu.regs.a.r();
    let r = a.wrapping_add(val);
    cpu.regs.a.w(r);
    cpu.set_flag(CPUFlags::Z, r == 0);
    cpu.set_flag(CPUFlags::N, false);
    cpu.set_flag(CPUFlags::H, ((a & 0x0F) + (val & 0x0F)) > 0x0F);
    cpu.set_flag(CPUFlags::C, (a as u16) + (val as u16) > 0xFF);
}
macro_rules! ld_into_reg {
    ($val: expr, $target_reg: expr) => {        
        $target_reg.w($val);
    };
}

fn ld_from_a_ind(addr: u16, cpu: &mut CPU) {
    let val = cpu.regs.a.r();
    cpu.write_byte(addr, val);    
}

#[allow(dead_code)]
fn create_isa <'i>() -> Vec<Instruction<'i>> {
    pushall!(
        [0x00, inst!("NOP", |cpu, op|{1})],
        [0x01, inst!("LD BC,nn", |cpu, op|{load_word_imm_u8!(cpu.regs.b, cpu.regs.c, cpu); 3})],
        [0x02, inst!("LD (BC),A", |cpu, op|{ld_from_a_ind(cpu.regs.bc(), cpu); 2})],
        [0x03, inst!("INC BC", |cpu, op|{inc_16!("BC", cpu); 2})],
        [0x04, inst!("INC B", |cpu, op| {inc!(cpu.regs.b, cpu, false); 1})], 
        [0x05, inst!("DEC B", |cpu, op|{dec!(cpu.regs.b, cpu, false); 1})],      
        [0x06, inst!("LD B,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.b, cpu); 2})], 
        
        [0x0A, inst!("LD A,(BC)", |cpu, op|{let addr = cpu.regs.bc(); let val = cpu.read_byte(addr); ld_into_reg!(val, cpu.regs.a); 2})],
        [0x0B, inst!("DEC BC", |cpu, op|{dec_16!("BC", cpu); 2})], 
        [0x0C, inst!("INC C", |cpu, op| {inc!(cpu.regs.c, cpu, false); 1})], 
        [0x0D, inst!("DEC C", |cpu, op|{dec!(cpu.regs.c, cpu, false); 1})],      
        [0x0E, inst!("LD C,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.c, cpu); 2})],

        [0x11, inst!("LD DE,nn", |cpu, op|{load_word_imm_u8!(cpu.regs.d, cpu.regs.e, cpu); 3})],  
        [0x12, inst!("LD (DE),A", |cpu, op|{ld_from_a_ind(cpu.regs.de(), cpu); 2})],
        [0x13, inst!("INC DE", |cpu, op|{inc_16!("DE", cpu); 2})],
        [0x14, inst!("INC D", |cpu, op| {inc!(cpu.regs.d, cpu, false); 1})],         
        [0x15, inst!("DEC D", |cpu, op|{dec!(cpu.regs.d, cpu, false); 1})],      
        [0x16, inst!("LD D,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.d, cpu); 2})], 
        [0x17, inst!("RLA", |cpu, op|{rotate_left!(cpu.regs.a, cpu); 1})],  
        
        [0x18, inst!("JR n", |cpu, op|{jump_cond_imm(cpu, JumpImmCond::None, JumpImmMode::IntOffset); 2})],
        [0x1A, inst!("LD A,(DE)", |cpu, op|{let addr = cpu.regs.de(); let val = cpu.read_byte(addr); ld_into_reg!(val, cpu.regs.a); 2})],       
        [0x1B, inst!("DEC DE", |cpu, op|{dec_16!("DE", cpu); 2})], 
        [0x1C, inst!("INC E", |cpu, op| {inc!(cpu.regs.e, cpu, false); 1})], 
        [0x1D, inst!("DEC E", |cpu, op|{dec!(cpu.regs.e, cpu, false); 1})],      
        [0x1E, inst!("LD E,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.e, cpu); 2})],

        [0x20, inst!("JR NZ,n", |cpu, op|{if jump_cond_imm(cpu, JumpImmCond::NZ, JumpImmMode::IntOffset){3} else {2}})],
        [0x21, inst!("LD HL,nn", |cpu, op|{load_word_imm_u8!(cpu.regs.h, cpu.regs.l, cpu); 3})],   
        [0x22, inst!("LDI (HL+), A", |cpu, op|{store_into_hl(cpu.regs.a.r(), cpu); inc_16!("HL", cpu); 3})],   
        [0x23, inst!("INC HL", |cpu, op|{inc_16!("HL", cpu); 2})],     
        [0x24, inst!("INC H", |cpu, op| {inc!(cpu.regs.h, cpu, false); 1})], 
        [0x25, inst!("DEC H", |cpu, op|{dec!(cpu.regs.h, cpu, false); 1})],
        [0x26, inst!("LD H,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.h, cpu); 2})],  
        
        [0x28, inst!("JR Z,n", |cpu, op|{if jump_cond_imm(cpu, JumpImmCond::Z, JumpImmMode::IntOffset){3} else {2}})],    
        [0x2A, inst!("LDI A,(HL+)", |cpu, op|{store_hl_into_a(cpu); inc_16!("HL", cpu); 3})],            
        [0x2B, inst!("DEC HL", |cpu, op|{dec_16!("HL", cpu); 2})], 
        [0x2C, inst!("INC L", |cpu, op| {inc!(cpu.regs.l, cpu, false); 1})], 
        [0x2D, inst!("DEC L", |cpu, op|{dec!(cpu.regs.l, cpu, false); 1})],
        [0x2E, inst!("LD L,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.l, cpu); 2})],
        [0x2F, inst!("CPL A", |cpu, op|{complement!(cpu.regs.a, cpu); 1})],

        [0x30, inst!("JR NC,n", |cpu, op|{if jump_cond_imm(cpu, JumpImmCond::NC, JumpImmMode::IntOffset){3} else {2}})],  
        [0x31, inst!("LD SP,nn", |cpu, op|{load_word_imm_u16!(cpu.sp, cpu); 3})],
        [0x32, inst!("LDD (HL-),A", |cpu, op|{store_into_hl(cpu.regs.a.r(), cpu); dec_16!("HL", cpu); 3})], 
        [0x33, inst!("INC SP", |cpu, op|{inc_16!("SP", cpu); 2})],           
        [0x34, inst!("INC (HL)", |cpu, op| {inc!(cpu.regs.l, cpu, true); 1})], 
        [0x35, inst!("DEC (HL)", |cpu, op|{dec!(cpu.regs.l, cpu, true); 3})],

        [0x36, inst!("LD (HL),#", |cpu, op|{store_into_hl(cpu.fetch_byte_immediate(), cpu); 3})],
        [0x38, inst!("JR C,n", |cpu, op|{if jump_cond_imm(cpu, JumpImmCond::C, JumpImmMode::IntOffset){3} else {2}})], 
        [0x3A, inst!("LDD A,(HL-)", |cpu, op|{store_hl_into_a(cpu); dec_16!("HL", cpu); 3})],        
        [0x3B, inst!("DEC SP", |cpu, op|{dec_16!("SP", cpu); 2})], 
        [0x3C, inst!("INC A", |cpu, op| {inc!(cpu.regs.a, cpu, false); 1})], 
        [0x3D, inst!("DEC L", |cpu, op|{dec!(cpu.regs.a, cpu, false); 1})],
        [0x3E, inst!("LD A,n", |cpu, op|{let val = cpu.fetch_byte_immediate(); ld_into_reg!(val, cpu.regs.a); 2})],

        [0x47, inst!("LD B,A", |cpu, op|{ld_into_reg!(cpu.regs.a.r(), cpu.regs.b); 1})],
        [0x4F, inst!("LD C,A", |cpu, op|{ld_into_reg!(cpu.regs.a.r(), cpu.regs.c); 1})],

        [0x57, inst!("LD D,A", |cpu, op|{ld_into_reg!(cpu.regs.a.r(), cpu.regs.d); 1})],
        [0x5F, inst!("LD E,A", |cpu, op|{ld_into_reg!(cpu.regs.a.r(), cpu.regs.e); 1})],

        [0x60, inst!("LD H,B", |cpu, op|{ld_into_reg!(cpu.regs.b.r(), cpu.regs.h); 1})],
        [0x61, inst!("LD H,C", |cpu, op|{ld_into_reg!(cpu.regs.c.r(), cpu.regs.h); 1})],
        [0x62, inst!("LD H,D", |cpu, op|{ld_into_reg!(cpu.regs.d.r(), cpu.regs.h); 1})],
        [0x63, inst!("LD H,E", |cpu, op|{ld_into_reg!(cpu.regs.e.r(), cpu.regs.h); 1})],
        [0x64, inst!("LD H,H", |cpu, op|{let val = cpu.regs.h.r(); ld_into_reg!(val, cpu.regs.h); 1})],
        [0x65, inst!("LD H,L", |cpu, op|{ld_into_reg!(cpu.regs.l.r(), cpu.regs.h); 1})],
        [0x66, inst!("LD H,(HL)", |cpu, op|{let val = cpu.read_byte(cpu.regs.hl()); ld_into_reg!(val, cpu.regs.h); 2})],
        [0x67, inst!("LD H,A", |cpu, op|{ld_into_reg!(cpu.regs.a.r(), cpu.regs.h); 1})],
        
        [0x68, inst!("LD L,B", |cpu, op|{ld_into_reg!(cpu.regs.b.r(), cpu.regs.l); 1})],
        [0x69, inst!("LD L,C", |cpu, op|{ld_into_reg!(cpu.regs.c.r(), cpu.regs.l); 1})],
        [0x6A, inst!("LD L,D", |cpu, op|{ld_into_reg!(cpu.regs.d.r(), cpu.regs.l); 1})],
        [0x6B, inst!("LD L,E", |cpu, op|{ld_into_reg!(cpu.regs.e.r(), cpu.regs.l); 1})],
        [0x6C, inst!("LD L,H", |cpu, op|{ld_into_reg!(cpu.regs.h.r(), cpu.regs.l); 1})],
        [0x6D, inst!("LD L,L", |cpu, op|{let val = cpu.regs.l.r(); ld_into_reg!(val, cpu.regs.l); 1})],
        [0x6E, inst!("LD L,(HL)", |cpu, op|{let val = cpu.read_byte(cpu.regs.hl()); ld_into_reg!(val, cpu.regs.l); 1})],
        [0x6F, inst!("LD L,A", |cpu, op|{ld_into_reg!(cpu.regs.a.r(), cpu.regs.l); 1})],

        [0x70, inst!("LD (HL),B", |cpu, op|{store_into_hl(cpu.regs.b.r(), cpu); 2})],
        [0x71, inst!("LD (HL),C", |cpu, op|{store_into_hl(cpu.regs.c.r(), cpu); 2})],
        [0x72, inst!("LD (HL),D", |cpu, op|{store_into_hl(cpu.regs.d.r(), cpu); 2})],
        [0x73, inst!("LD (HL),E", |cpu, op|{store_into_hl(cpu.regs.e.r(), cpu); 2})],
        [0x74, inst!("LD (HL),H", |cpu, op|{store_into_hl(cpu.regs.h.r(), cpu); 2})],
        [0x75, inst!("LD (HL),L", |cpu, op|{store_into_hl(cpu.regs.l.r(), cpu); 2})],
        [0x77, inst!("LD (HL),A", |cpu, op|{store_into_hl(cpu.regs.a.r(), cpu); 2})],

        [0x78, inst!("LD A,B", |cpu, op|{ld_into_reg!(cpu.regs.b.r(), cpu.regs.a); 1})],
        [0x79, inst!("LD A,C", |cpu, op|{ld_into_reg!(cpu.regs.c.r(), cpu.regs.a); 1})],
        [0x7A, inst!("LD A,D", |cpu, op|{ld_into_reg!(cpu.regs.d.r(), cpu.regs.a); 1})],
        [0x7B, inst!("LD A,E", |cpu, op|{ld_into_reg!(cpu.regs.e.r(), cpu.regs.a); 1})],
        [0x7C, inst!("LD A,H", |cpu, op|{ld_into_reg!(cpu.regs.h.r(), cpu.regs.a); 1})],
        [0x7D, inst!("LD A,L", |cpu, op|{ld_into_reg!(cpu.regs.l.r(), cpu.regs.a); 1})],
        [0x7E, inst!("LD A,(HL)", |cpu, op|{let addr = cpu.regs.hl(); let val = cpu.read_byte(addr); ld_into_reg!(val, cpu.regs.a); 2})],
        [0x7F, inst!("LD A,A", |cpu, op|{let val = cpu.regs.a.r(); ld_into_reg!(val, cpu.regs.a); 1})],

        [0x80, inst!("ADD A,B", |cpu, op|{add_to_a(cpu.regs.b.r(), cpu); 1})],
        [0x81, inst!("ADD A,C", |cpu, op|{add_to_a(cpu.regs.c.r(), cpu); 1})],
        [0x82, inst!("ADD A,D", |cpu, op|{add_to_a(cpu.regs.d.r(), cpu); 1})],
        [0x83, inst!("ADD A,E", |cpu, op|{add_to_a(cpu.regs.e.r(), cpu); 1})],
        [0x84, inst!("ADD A,H", |cpu, op|{add_to_a(cpu.regs.h.r(), cpu); 1})],
        [0x85, inst!("ADD A,L", |cpu, op|{add_to_a(cpu.regs.l.r(), cpu); 1})],
        [0x86, inst!("ADD A,(HL)", |cpu, op|{let addr = cpu.regs.hl(); add_to_a(cpu.read_byte(addr), cpu); 2})],
        [0x87, inst!("ADD A,A", |cpu, op|{add_to_a(cpu.regs.a.r(), cpu); 1})],

        [0x88, inst!("ADC A,B", |cpu, op|{add_carry(cpu.regs.b.r(), cpu); 1})],
        [0x89, inst!("ADC A,C", |cpu, op|{add_carry(cpu.regs.c.r(), cpu); 1})],
        [0x8A, inst!("ADC A,D", |cpu, op|{add_carry(cpu.regs.d.r(), cpu); 1})],
        [0x8B, inst!("ADC A,E", |cpu, op|{add_carry(cpu.regs.e.r(), cpu); 1})],
        [0x8C, inst!("ADC A,H", |cpu, op|{add_carry(cpu.regs.h.r(), cpu); 1})],
        [0x8D, inst!("ADC A,L", |cpu, op|{add_carry(cpu.regs.l.r(), cpu); 1})],
        [0x8E, inst!("ADC A,(HL)", |cpu, op|{let hl = cpu.regs.hl(); add_carry(cpu.read_byte(hl), cpu); 2})],
        [0x8F, inst!("ADC A,A", |cpu, op|{add_carry(cpu.regs.a.r(), cpu); 1})],

        [0x90, inst!("SUB A,B", |cpu, op|{sub_to_a(cpu.regs.b.r(), cpu); 1})],
        [0x91, inst!("SUB A,C", |cpu, op|{sub_to_a(cpu.regs.c.r(), cpu); 1})],
        [0x92, inst!("SUB A,D", |cpu, op|{sub_to_a(cpu.regs.d.r(), cpu); 1})],
        [0x93, inst!("SUB A,E", |cpu, op|{sub_to_a(cpu.regs.e.r(), cpu); 1})],
        [0x94, inst!("SUB A,H", |cpu, op|{sub_to_a(cpu.regs.h.r(), cpu); 1})],
        [0x95, inst!("SUB A,L", |cpu, op|{sub_to_a(cpu.regs.l.r(), cpu); 1})],
        [0x96, inst!("SUB A,(HL)", |cpu, op|{let addr = cpu.regs.hl(); sub_to_a(cpu.read_byte(addr), cpu); 2})],
        [0x97, inst!("SUB A,A", |cpu, op|{sub_to_a(cpu.regs.a.r(), cpu); 1})],
        
        [0xA0, inst!("AND A,B", |cpu, op|{and(cpu.regs.b.r(), cpu); 1})],
        [0xA1, inst!("AND A,C", |cpu, op|{and(cpu.regs.c.r(), cpu); 1})],
        [0xA2, inst!("AND A,D", |cpu, op|{and(cpu.regs.d.r(), cpu); 1})],
        [0xA3, inst!("AND A,E", |cpu, op|{and(cpu.regs.e.r(), cpu); 1})],
        [0xA4, inst!("AND A,H", |cpu, op|{and(cpu.regs.h.r(), cpu); 1})],
        [0xA5, inst!("AND A,L", |cpu, op|{and(cpu.regs.l.r(), cpu); 1})],
        [0xA6, inst!("AND A,(HL)", |cpu, op|{let hl = cpu.regs.hl(); and(cpu.read_byte(hl), cpu); 2})],
        [0xA7, inst!("AND A,A", |cpu, op|{and(cpu.regs.a.r(), cpu); 1})],

        [0xA8, inst!("XOR A,B", |cpu, op|{xor(cpu.regs.b.r(), cpu); 1})],
        [0xA9, inst!("XOR A,C", |cpu, op|{xor(cpu.regs.c.r(), cpu); 1})],
        [0xAA, inst!("XOR A,D", |cpu, op|{xor(cpu.regs.d.r(), cpu); 1})],
        [0xAB, inst!("XOR A,E", |cpu, op|{xor(cpu.regs.e.r(), cpu); 1})],
        [0xAC, inst!("XOR A,H", |cpu, op|{xor(cpu.regs.h.r(), cpu); 1})],
        [0xAD, inst!("XOR A,L", |cpu, op|{xor(cpu.regs.l.r(), cpu); 1})],
        [0xAE, inst!("XOR A,(HL)", |cpu, op|{let hl = cpu.regs.hl(); xor(cpu.read_byte(hl), cpu); 2})],
        [0xAF, inst!("XOR A,A", |cpu, op|{xor(cpu.regs.a.r(), cpu); 1})],

        [0xB0, inst!("OR A,B", |cpu, op|{or(cpu.regs.b.r(), cpu); 1})],
        [0xB1, inst!("OR A,C", |cpu, op|{or(cpu.regs.c.r(), cpu); 1})],
        [0xB2, inst!("OR A,D", |cpu, op|{or(cpu.regs.d.r(), cpu); 1})],
        [0xB3, inst!("OR A,E", |cpu, op|{or(cpu.regs.e.r(), cpu); 1})],
        [0xB4, inst!("OR A,H", |cpu, op|{or(cpu.regs.h.r(), cpu); 1})],
        [0xB5, inst!("OR A,L", |cpu, op|{or(cpu.regs.l.r(), cpu); 1})],
        [0xB6, inst!("OR A,(HL)", |cpu, op|{let hl = cpu.regs.hl(); or(cpu.read_byte(hl), cpu); 2})],
        [0xB7, inst!("OR A,A", |cpu, op|{or(cpu.regs.a.r(), cpu); 1})],

        [0xB8, inst!("CP B", |cpu, op|{compare_with_a(cpu.regs.b.r(), cpu); 1})],
        [0xB9, inst!("CP C", |cpu, op|{compare_with_a(cpu.regs.c.r(), cpu); 1})],
        [0xBA, inst!("CP D", |cpu, op|{compare_with_a(cpu.regs.d.r(), cpu); 1})],
        [0xBB, inst!("CP E", |cpu, op|{compare_with_a(cpu.regs.e.r(), cpu); 1})],
        [0xBC, inst!("CP H", |cpu, op|{compare_with_a(cpu.regs.h.r(), cpu); 1})],
        [0xBD, inst!("CP L", |cpu, op|{compare_with_a(cpu.regs.l.r(), cpu); 1})],
        [0xBE, inst!("CP (HL)", |cpu, op|{let val = cpu.read_byte(cpu.regs.hl()); compare_with_a(val, cpu); 2})],
        [0xBF, inst!("CP A", |cpu, op|{compare_with_a(cpu.regs.a.r(), cpu); 1})],

        [0xC1, inst!("POP BC", |cpu, op|{pop_into!(cpu.regs.b, cpu.regs.c, cpu);3})],
        [0xC3, inst!("JP nn", |cpu, op|{jump_cond_imm(cpu, JumpImmCond::None, JumpImmMode::Immediate); 3})],
        [0xC4, inst!("CALL NZ,nn", |cpu, op|{call_cond(cpu, JumpImmCond::NZ);3})],
        [0xC5, inst!("PUSH BC", |cpu, op|{let val = cpu.regs.bc();cpu.push_word(val); 4})],
        [0xC6, inst!("ADD A,#", |cpu, op|{add_to_a(cpu.fetch_byte_immediate(), cpu); 2})],
        [0xC7, inst!("RST 0x00", |cpu, op|{reset(op, cpu); 8})],
        [0xC9, inst!("RET", |cpu, op|{let target_addr = cpu.pop_word(); jump(target_addr, cpu); 2})],
        [0xCC, inst!("CALL Z,nn", |cpu, op|{call_cond(cpu, JumpImmCond::Z);3})],
        [0xCD, inst!("CALL nn", |cpu, op|{call_cond(cpu, JumpImmCond::None); 3})],
        [0xCF, inst!("RST 0x08", |cpu, op|{reset(op, cpu); 8})],
        
        [0xD1, inst!("POP DE", |cpu, op|{pop_into!(cpu.regs.d, cpu.regs.e, cpu);3})],
        [0xD4, inst!("CALL C,nn", |cpu, op|{call_cond(cpu, JumpImmCond::C);3})],
        [0xD5, inst!("PUSH DE", |cpu, op|{let val = cpu.regs.de();cpu.push_word(val); 4})],
        [0xD6, inst!("SUB A,#", |cpu, op|{sub_to_a(cpu.fetch_byte_immediate(), cpu); 2})],
        [0xD7, inst!("RST 0x10", |cpu, op|{reset(op, cpu); 8})],
        
        [0xDC, inst!("CALL NC,nn", |cpu, op|{call_cond(cpu, JumpImmCond::NC);3})],
        [0xDF, inst!("RST 0x18", |cpu, op|{reset(op, cpu); 8})],

        [0xE0, inst!("LD (0xFF00+n),A", |cpu, op|{let off = cpu.fetch_byte_immediate();ldh(cpu, off, false);3})],
        [0xE1, inst!("POP HL", |cpu, op|{pop_into!(cpu.regs.h, cpu.regs.l, cpu);3})],
        [0xE2, inst!("LD (0xFF00+C),A", |cpu, op|{let off = cpu.regs.c.r(); ldh(cpu, off, false);3})],
        [0xE5, inst!("PUSH HL", |cpu, op|{let val = cpu.regs.hl();cpu.push_word(val); 4})],
        [0xE6, inst!("AND A,#", |cpu, op|{and(cpu.fetch_byte_immediate(), cpu); 2})],
        [0xE7, inst!("RST 0x20", |cpu, op|{reset(op, cpu); 8})],
        [0xE9, inst!("JP (HL)", |cpu, op|{jump(cpu.regs.hl(), cpu); 1})],
        [0xEA, inst!("LD (nn),A", |cpu, op|{let addr = cpu.fetch_word_immediate(); ld_from_a_ind(addr, cpu); 4})],
        [0xEF, inst!("RST 0x28", |cpu, op|{reset(op, cpu); 8})],
        
        [0xF0, inst!("LD A,(0xFF00+n)", |cpu, op|{let off = cpu.fetch_byte_immediate(); ldh(cpu, off, true); 3})],
        [0xF1, inst!("POP AF", |cpu, op|{pop_into!(cpu.regs.a, cpu.regs.f, cpu);3})],
        [0xF2, inst!("LD A,(0xFF00+C)", |cpu, op|{let off = cpu.regs.c.r(); ldh(cpu, off, true); 3})],
        [0xF3, inst!("DI", |cpu, op|{cpu.disable_interrupts_delayed(); 1})],
        [0xF5, inst!("PUSH AF", |cpu, op|{let val = cpu.regs.af();cpu.push_word(val); 4})],
        [0xF6, inst!("OR A,#", |cpu, op|{or(cpu.fetch_byte_immediate(), cpu); 2})],
        [0xF7, inst!("RST 0x30", |cpu, op|{reset(op, cpu); 8})],
        [0xFA, inst!("LD A,(nn)", |cpu, op|{let addr = cpu.fetch_word_immediate(); let val = cpu.read_byte(addr); ld_into_reg!(val, cpu.regs.a); 4})],
        [0xFB, inst!("EI", |cpu, op|{cpu.enable_interrupts_delayed(); 1})],
        [0xFE, inst!("CP n", |cpu, op|{compare_with_a(cpu.fetch_byte_immediate(), cpu); 2})],
        [0xFF, inst!("RST 0x038", |cpu, op|{reset(op, cpu); 8})]
    )
}

fn test_bit(opcode: u8, cpu: &mut CPU) {
    let bit_to_test = (opcode & 0b00111000) >> 3;
    let register = (opcode & 0b00000111);
    let val: bool;
    match register {
        0b000 => {val = cpu.regs.b.is_bit_set(bit_to_test);}
        0b001 => {val = cpu.regs.c.is_bit_set(bit_to_test);}
        0b010 => {val = cpu.regs.d.is_bit_set(bit_to_test);}
        0b011 => {val = cpu.regs.e.is_bit_set(bit_to_test);}
        0b100 => {val = cpu.regs.h.is_bit_set(bit_to_test);}
        0b101 => {val = cpu.regs.l.is_bit_set(bit_to_test);}
        0b111 => {val = cpu.regs.a.is_bit_set(bit_to_test);}
        _ => {panic!("Unrecognized register in bit check instruction {:2X}", opcode)}
    }
    cpu.set_flag(CPUFlags::Z, val == false);
    cpu.set_flag(CPUFlags::N, false);
    cpu.set_flag(CPUFlags::H, true);    
}

fn perform_swap(val: u8) -> u8 {
    ((val & 0xF0) >> 4) + ((val & 0xF) << 4)
}

macro_rules! swap_halves {
    ($target_reg: expr, $cpu: expr) => {
        let val = $target_reg.r();
        let swapped = perform_swap(val);
        if perform_swap(swapped) != val {panic!("Swap halves is bugged!")}
        $target_reg.w(swapped);
        $cpu.set_flag(CPUFlags::Z, swapped == 0);
    };
}

fn swap_halves_ind(addr: u16, cpu: &mut CPU) {
    let val = cpu.read_byte(addr);
    let swapped = perform_swap(val);
    if perform_swap(swapped) != val {panic!("Swap halves is bugged!")}
    cpu.write_byte(addr, swapped);

    cpu.set_flag(CPUFlags::Z, swapped == 0);
}

#[allow(dead_code)]
fn create_bitwise_isa <'i>() -> Vec<Instruction<'i>> {
    pushall!(

        [0x10, inst!("RL B", |cpu, op|{rotate_left!(cpu.regs.b, cpu); 2})],
        [0x11, inst!("RL C", |cpu, op|{rotate_left!(cpu.regs.c, cpu); 2})],
        [0x12, inst!("RL D", |cpu, op|{rotate_left!(cpu.regs.d, cpu); 2})],
        [0x13, inst!("RL E", |cpu, op|{rotate_left!(cpu.regs.e, cpu); 2})],
        [0x14, inst!("RL H", |cpu, op|{rotate_left!(cpu.regs.h, cpu); 2})],
        [0x15, inst!("RL L", |cpu, op|{rotate_left!(cpu.regs.l, cpu); 2})],
        [0x16, inst!("RL (HL)", |cpu, op|{rotate_left_ind(cpu.regs.hl(), cpu); 4})],
        [0x17, inst!("RL A", |cpu, op|{rotate_left!(cpu.regs.a, cpu); 2})],

        [0x30, inst!("SWAP B", |cpu, op|{swap_halves!(cpu.regs.b, cpu); 2})],
        [0x31, inst!("SWAP C", |cpu, op|{swap_halves!(cpu.regs.c, cpu); 2})],
        [0x32, inst!("SWAP D", |cpu, op|{swap_halves!(cpu.regs.d, cpu); 2})],
        [0x33, inst!("SWAP E", |cpu, op|{swap_halves!(cpu.regs.e, cpu); 2})],
        [0x34, inst!("SWAP H", |cpu, op|{swap_halves!(cpu.regs.h, cpu); 2})],
        [0x35, inst!("SWAP L", |cpu, op|{swap_halves!(cpu.regs.l, cpu); 2})],
        [0x36, inst!("SWAP (HL)", |cpu, op|{swap_halves_ind(cpu.regs.hl(), cpu); 4})],
        [0x37, inst!("SWAP A", |cpu, op|{swap_halves!(cpu.regs.a, cpu); 2})],

        [0x40, inst!("BIT 0,B", |cpu, op|{test_bit(op, cpu); 2})],
        [0x41, inst!("BIT 0,C", |cpu, op|{test_bit(op, cpu); 2})],
        [0x42, inst!("BIT 0,D", |cpu, op|{test_bit(op, cpu); 2})],
        [0x43, inst!("BIT 0,E", |cpu, op|{test_bit(op, cpu); 2})],
        [0x44, inst!("BIT 0,H", |cpu, op|{test_bit(op, cpu); 2})],
        [0x45, inst!("BIT 0,L", |cpu, op|{test_bit(op, cpu); 2})],
        [0x46, inst!("Unimp", |cpu, op|{2})],
        [0x47, inst!("BIT 0,A", |cpu, op|{test_bit(op, cpu); 2})],

        [0x48, inst!("BIT 1,B", |cpu, op|{test_bit(op, cpu); 2})],
        [0x49, inst!("BIT 1,C", |cpu, op|{test_bit(op, cpu); 2})],
        [0x4A, inst!("BIT 1,D", |cpu, op|{test_bit(op, cpu); 2})],
        [0x4B, inst!("BIT 1,E", |cpu, op|{test_bit(op, cpu); 2})],
        [0x4C, inst!("BIT 1,H", |cpu, op|{test_bit(op, cpu); 2})],
        [0x4D, inst!("BIT 1,L", |cpu, op|{test_bit(op, cpu); 2})],
        [0x4E, inst!("Unimp", |cpu, op|{2})],
        [0x4F, inst!("BIT 1,A", |cpu, op|{test_bit(op, cpu); 2})],

        [0x50, inst!("BIT 2,B", |cpu, op|{test_bit(op, cpu); 2})],
        [0x51, inst!("BIT 2,C", |cpu, op|{test_bit(op, cpu); 2})],
        [0x52, inst!("BIT 2,D", |cpu, op|{test_bit(op, cpu); 2})],
        [0x53, inst!("BIT 2,E", |cpu, op|{test_bit(op, cpu); 2})],
        [0x54, inst!("BIT 2,H", |cpu, op|{test_bit(op, cpu); 2})],
        [0x55, inst!("BIT 2,L", |cpu, op|{test_bit(op, cpu); 2})],
        [0x56, inst!("Unimp", |cpu, op|{2})],
        [0x57, inst!("BIT 2,A", |cpu, op|{test_bit(op, cpu); 2})],

        [0x58, inst!("BIT 3,B", |cpu, op|{test_bit(op, cpu); 2})],
        [0x59, inst!("BIT 3,C", |cpu, op|{test_bit(op, cpu); 2})],
        [0x5A, inst!("BIT 3,D", |cpu, op|{test_bit(op, cpu); 2})],
        [0x5B, inst!("BIT 3,E", |cpu, op|{test_bit(op, cpu); 2})],
        [0x5C, inst!("BIT 3,H", |cpu, op|{test_bit(op, cpu); 2})],
        [0x5D, inst!("BIT 3,L", |cpu, op|{test_bit(op, cpu); 2})],
        [0x5E, inst!("Unimp", |cpu, op|{2})],
        [0x5F, inst!("BIT 3,A", |cpu, op|{test_bit(op, cpu); 2})],

        [0x60, inst!("BIT 4,B", |cpu, op|{test_bit(op, cpu); 2})],
        [0x61, inst!("BIT 4,C", |cpu, op|{test_bit(op, cpu); 2})],
        [0x62, inst!("BIT 4,D", |cpu, op|{test_bit(op, cpu); 2})],
        [0x63, inst!("BIT 4,E", |cpu, op|{test_bit(op, cpu); 2})],
        [0x64, inst!("BIT 4,H", |cpu, op|{test_bit(op, cpu); 2})],
        [0x65, inst!("BIT 4,L", |cpu, op|{test_bit(op, cpu); 2})],
        [0x66, inst!("Unimp", |cpu, op|{2})],
        [0x67, inst!("BIT 4,A", |cpu, op|{test_bit(op, cpu); 2})],

        [0x60, inst!("BIT 4,B", |cpu, op|{test_bit(op, cpu); 2})],
        [0x61, inst!("BIT 4,C", |cpu, op|{test_bit(op, cpu); 2})],
        [0x62, inst!("BIT 4,D", |cpu, op|{test_bit(op, cpu); 2})],
        [0x63, inst!("BIT 4,E", |cpu, op|{test_bit(op, cpu); 2})],
        [0x64, inst!("BIT 4,H", |cpu, op|{test_bit(op, cpu); 2})],
        [0x65, inst!("BIT 4,L", |cpu, op|{test_bit(op, cpu); 2})],
        [0x66, inst!("Unimp", |cpu, op|{2})],
        [0x67, inst!("BIT 4,A", |cpu, op|{test_bit(op, cpu); 2})],

        [0x68, inst!("BIT 5,B", |cpu, op|{test_bit(op, cpu); 2})],
        [0x69, inst!("BIT 5,C", |cpu, op|{test_bit(op, cpu); 2})],
        [0x6A, inst!("BIT 5,D", |cpu, op|{test_bit(op, cpu); 2})],
        [0x6B, inst!("BIT 5,E", |cpu, op|{test_bit(op, cpu); 2})],
        [0x6C, inst!("BIT 5,H", |cpu, op|{test_bit(op, cpu); 2})],
        [0x6D, inst!("BIT 5,L", |cpu, op|{test_bit(op, cpu); 2})],
        [0x6E, inst!("Unimp", |cpu, op|{2})],
        [0x6F, inst!("BIT 5,A", |cpu, op|{test_bit(op, cpu); 2})],

        [0x70, inst!("BIT 6,B", |cpu, op|{test_bit(op, cpu); 2})],
        [0x71, inst!("BIT 6,C", |cpu, op|{test_bit(op, cpu); 2})],
        [0x72, inst!("BIT 6,D", |cpu, op|{test_bit(op, cpu); 2})],
        [0x73, inst!("BIT 6,E", |cpu, op|{test_bit(op, cpu); 2})],
        [0x74, inst!("BIT 6,H", |cpu, op|{test_bit(op, cpu); 2})],
        [0x75, inst!("BIT 6,L", |cpu, op|{test_bit(op, cpu); 2})],
        [0x76, inst!("Unimp", |cpu, op|{2})],
        [0x77, inst!("BIT 6,A", |cpu, op|{test_bit(op, cpu); 2})],

        [0x78, inst!("BIT 7,B", |cpu, op|{test_bit(op, cpu); 2})],
        [0x79, inst!("BIT 7,C", |cpu, op|{test_bit(op, cpu); 2})],
        [0x7A, inst!("BIT 7,D", |cpu, op|{test_bit(op, cpu); 2})],
        [0x7B, inst!("BIT 7,E", |cpu, op|{test_bit(op, cpu); 2})],
        [0x7C, inst!("BIT 7,H", |cpu, op|{test_bit(op, cpu); 2})],
        [0x7D, inst!("BIT 7,L", |cpu, op|{test_bit(op, cpu); 2})],
        [0x7E, inst!("Unimp", |cpu, op|{2})],
        [0x7F, inst!("BIT 7,A", |cpu, op|{test_bit(op, cpu); 2})]
    )
}