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

    pub fn execute(&mut self, cpu: &mut CPU, opcode: u8) -> u32 {
        let closure = &self.op;
        closure(cpu, opcode)
    }
}

pub struct InstructionSet<'i> {
    instruction_set: Vec<Instruction<'i>>
}

impl<'i> InstructionSet<'i> {
    pub fn new() -> Self {
        InstructionSet {
            instruction_set: create_isa()
        }
    }

    pub fn is_implemented(&self, opcode: u8) -> bool {
        self.instruction_set[opcode as usize].dissassembly != "Unimp"
    }

    pub fn exec(&mut self, cpu: &mut CPU, opcode: u8) -> u32 {
        self.instruction_set[opcode as usize].execute(cpu, opcode)
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

macro_rules! jp_imm_cond {
     ($cond:expr, $cpu:expr) => {{
        if $cond {
            let imm = $cpu.fetch_word_immediate();
            $cpu.pc.w(imm);
        }
    }}    
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

fn store_and_decrement(cpu: &mut CPU) {
    let hl = cpu.regs.hl();
    let a = cpu.regs.a.r();
    cpu.write_byte(hl, a);
    cpu.regs.hl_w(hl.wrapping_sub(1));    
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

#[allow(dead_code)]
fn create_isa <'i>() -> Vec<Instruction<'i>> {
    pushall!(
        [0x00, inst!( "NOP", |cpu, op|{1})],
        [0x01, inst!("LD BC,nn", |cpu, op|{load_word_imm_u8!(cpu.regs.b, cpu.regs.c, cpu); 3})],  
        [0x05, inst!("DEC B", |cpu, op|{dec!(cpu.regs.b, cpu, false); 1})],      
        [0x06, inst!("LD B,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.b, cpu); 2})],  
        [0x0D, inst!("DEC C", |cpu, op|{dec!(cpu.regs.c, cpu, false); 1})],      
        [0x0E, inst!("LD C,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.c, cpu); 2})],

        [0x11, inst!("LD DE,nn", |cpu, op|{load_word_imm_u8!(cpu.regs.d, cpu.regs.e, cpu); 3})],          
        [0x15, inst!("DEC D", |cpu, op|{dec!(cpu.regs.d, cpu, false); 1})],      
        [0x16, inst!("LD D,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.d, cpu); 2})],          
        [0x1D, inst!("DEC E", |cpu, op|{dec!(cpu.regs.e, cpu, false); 1})],      
        [0x1E, inst!("LD E,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.h, cpu); 2})],

        [0x21, inst!("LD HL,nn", |cpu, op|{load_word_imm_u8!(cpu.regs.h, cpu.regs.l, cpu); 3})],      
        [0x25, inst!("DEC H", |cpu, op|{dec!(cpu.regs.h, cpu, false); 1})],
        [0x26, inst!("LD H,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.h, cpu); 2})],              
        [0x2D, inst!("DEC E", |cpu, op|{dec!(cpu.regs.l, cpu, false); 1})],
        [0x2E, inst!("LD L,n", |cpu, op|{load_byte_imm_u8!(cpu.regs.l, cpu); 2})],

        [0x31, inst!("LD SP,nn", |cpu, op|{load_word_imm_u16!(cpu.sp, cpu); 3})],
        [0x32, inst!("LDD (HL-),A", |cpu, op|{store_and_decrement(cpu); 3})],            
        [0x35, inst!("DEC (HL)", |cpu, op|{dec!(cpu.regs.l, cpu, true); 3})],

        [0x88, inst!("ADC A,B", |cpu, op|{add_carry(cpu.regs.b.r(), cpu); 1})],
        [0x89, inst!("ADC A,C", |cpu, op|{add_carry(cpu.regs.c.r(), cpu); 1})],
        [0x8A, inst!("ADC A,D", |cpu, op|{add_carry(cpu.regs.d.r(), cpu); 1})],
        [0x8B, inst!("ADC A,E", |cpu, op|{add_carry(cpu.regs.e.r(), cpu); 1})],
        [0x8C, inst!("ADC A,H", |cpu, op|{add_carry(cpu.regs.h.r(), cpu); 1})],
        [0x8D, inst!("ADC A,L", |cpu, op|{add_carry(cpu.regs.l.r(), cpu); 1})],
        [0x8E, inst!("ADC A,(HL)", |cpu, op|{let hl = cpu.regs.hl(); add_carry(cpu.read_byte(hl), cpu); 2})],
        [0x8F, inst!("ADC A,A", |cpu, op|{add_carry(cpu.regs.a.r(), cpu); 1})],

        [0xA8, inst!("XOR A,B", |cpu, op|{xor(cpu.regs.b.r(), cpu); 1})],
        [0xA9, inst!("XOR A,C", |cpu, op|{xor(cpu.regs.c.r(), cpu); 1})],
        [0xAA, inst!("XOR A,D", |cpu, op|{xor(cpu.regs.d.r(), cpu); 1})],
        [0xAB, inst!("XOR A,E", |cpu, op|{xor(cpu.regs.e.r(), cpu); 1})],
        [0xAC, inst!("XOR A,H", |cpu, op|{xor(cpu.regs.h.r(), cpu); 1})],
        [0xAD, inst!("XOR A,L", |cpu, op|{xor(cpu.regs.l.r(), cpu); 1})],
        [0xAE, inst!("XOR A,(HL)", |cpu, op|{let hl = cpu.regs.hl(); xor(cpu.read_byte(hl), cpu); 2})],
        [0xAF, inst!("XOR A,A", |cpu, op|{xor(cpu.regs.a.r(), cpu); 1})],

        [0xC3, inst!( "JP nn", |cpu, op|{jp_imm_cond!(true, cpu); 3})]
    )
}