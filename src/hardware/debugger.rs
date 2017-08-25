use hardware::cpu::CPU;
use std::collections::HashSet;

#[derive(PartialEq)]
enum DebuggerState {
    RUN,
    STEP,
}

pub struct Debugger {
    breakpoints: HashSet<u16>,
    state: DebuggerState
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            breakpoints: create_breakpoints(),
            state: DebuggerState::RUN
        }
    }

    pub fn stop_if_needed(&mut self, pc: u16, cpu: &CPU) {
        if self.breakpoints.contains(&pc) || self.state == DebuggerState::STEP {
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

        let mut good_command = false;
        println!("Debug Command: ");
        let mut command: String = read!();
        while !good_command {
            
        
            match &*command {
                "n" => {self.state = DebuggerState::STEP; good_command = true}
                "c" => {self.state = DebuggerState::RUN; good_command = true}                
                _ => {print!("Debug Command: "); command = read!();}
            }
        }
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
    let breakpoints = hash![
        0x68
    ];
    breakpoints
}
