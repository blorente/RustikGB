use cpu::cpu::CPU;

struct Instruction<'i> {
    pub dissassembly : &'static str,
    operand_num : u8,
    op : Box<Fn(&mut CPU, u8) -> u32 + 'i>,
}

impl<'i> Instruction<'i> {
    pub fn new<F: Fn(&mut CPU, u8) -> u32 + 'i> (dissassembly: &'static str, operand_num : u8, func: F) -> Instruction<'i> {
        Instruction {
            dissassembly: dissassembly,
            operand_num: operand_num,
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
    ($x:expr, $y:expr, $f:expr) => {{
        #[allow(dead_code)]
        let inst = Instruction::new($x, $y, $f);
        inst
    }}    
}

macro_rules! pushall {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[allow(dead_code)]
fn create_isa <'i>() -> Vec<Instruction<'i>> {
    pushall!(
        inst!("NOP", 0, |cpu, x|{1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),         
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}),
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1}), 
        inst!("Unimp", 0, |cpu, x|{println!("Unimplemented inst with params {}", x); 1})
    )
}