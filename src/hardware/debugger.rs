use hardware::cpu::CPU;
use std::collections::HashSet;
use hardware::instructions::InstructionSet;

#[derive(PartialEq)]
enum DebuggerState {
    RUN,
    STEP,
}

pub struct Debugger {
    breakpoints: HashSet<u16>,
    state: DebuggerState,
    activated: bool,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            breakpoints: create_breakpoints(),
            state: DebuggerState::RUN,
            activated: false,
        }
    }

    pub fn stop_if_needed(&mut self, pc: u16, cpu: &CPU, instruction_set: &InstructionSet) {
        if self.activated {
            let mut opcode = cpu.read_byte(pc);
            let mut bitwise = false;
            if opcode == 0xCB {opcode = cpu.read_byte(pc + 1); bitwise = true;}
            println!("PC: {:04X}, Opcode {:02X}: {}",
                    pc,
                    opcode,
                    instruction_set.print_instr(opcode, bitwise)); 
        }

        if self.breakpoints.contains(&pc) || self.state == DebuggerState::STEP {
            self.activated = true;
            self.stop_and_ask(pc, cpu);
        }
    }

    fn stop_and_ask(&mut self, pc: u16, cpu: &CPU) {
        println!("DEBUGGER");
        println!("================");
        println!("Program stopped at address 0x{:0X} with opcode {:0X}", 
                pc,
                cpu.read_byte(pc));
        println!("Processor state:\n{}", cpu);

        let mut stop_asking = false;
        println!("Debug Command: ");
        let mut command: String = read!("{}\n");
        while !stop_asking {            
            match &command[..1] {
                "n" => {self.state = DebuggerState::STEP; stop_asking = true}
                "c" => {self.state = DebuggerState::RUN; stop_asking = true}   
                "p" => {
                    let addr : u16 = u16::from_str_radix(&command[4..8], 16).unwrap();
                    self.print_addr(addr, cpu);                    
                    }             
                _ => {}
            }

            if !stop_asking {print!("Debug Command: "); command = read!("{}\n");}
        }
    }

    fn print_addr(&self, addr: u16, cpu: &CPU) {
        let val = cpu.read_byte(addr);
        println!("Addr {:04X} contains {:02X}", addr, val);
    }
}

macro_rules! hash {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = HashSet::new();
            $(
                temp_vec.insert($x);
            )*
            temp_vec
        }
    };
}

fn create_breakpoints() -> HashSet<u16> {
    hash![
        0x70
    ]
}
