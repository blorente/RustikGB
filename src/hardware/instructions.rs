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
            let addr = $cpu.pc.r();
            let imm = $cpu.read_word(addr + 1);
            $cpu.pc.w(imm); 
        }
    }}    
}

fn add_carry(other : u8, cpu : &mut CPU) {
    let carry = if cpu.is_flag_set(CPUFlags::C) {1} else {0};
    let a : u8 = cpu.regs.a.r();
    let res : u16 = a as u16 + other as u16 + carry as u16;
    let res_trunc : u8 = (res & 0xF) as u8;
    cpu.set_flag(CPUFlags::Z, res_trunc == 0);
    cpu.set_flag(CPUFlags::N, false);
    cpu.set_flag(CPUFlags::H, (a & 0xF) + (other & 0xF) + carry > 0xF);
    cpu.set_flag(CPUFlags::C, res > 0xFF);
    cpu.regs.a.w(res_trunc);
}

#[allow(dead_code)]
fn create_isa <'i>() -> Vec<Instruction<'i>> {
    pushall!(
       [0x00, inst!( "NOP", |cpu, op|{1})],
       [0x88, inst!("ADC A,B", |cpu, op|{add_carry(cpu.regs.b.r(), cpu); 1})],
       [0x89, inst!("ADC A,C", |cpu, op|{add_carry(cpu.regs.c.r(), cpu); 1})],
       [0x8A, inst!("ADC A,D", |cpu, op|{add_carry(cpu.regs.d.r(), cpu); 1})],
       [0x8B, inst!("ADC A,E", |cpu, op|{add_carry(cpu.regs.e.r(), cpu); 1})],
       [0x8C, inst!("ADC A,H", |cpu, op|{add_carry(cpu.regs.h.r(), cpu); 1})],
       [0x8D, inst!("ADC A,L", |cpu, op|{add_carry(cpu.regs.l.r(), cpu); 1})],
       [0x8E, inst!("ADC A,(HL)", |cpu, op|{let hl = cpu.regs.hl(); add_carry(cpu.read_byte(hl), cpu); 2})],
       [0x8F, inst!("ADC A,A", |cpu, op|{add_carry(cpu.regs.a.r(), cpu); 1})],
       [0xC3, inst!( "JP nn", |cpu, op|{jp_imm_cond!(true, cpu); 3})]
    )
}