mod cpu;

fn main() {
    println!("Hello, world!");
    let processor : cpu::cpu::CPU = Default::default();

    let instr = [
        cpu::cpu::Instruction::new("NOP", 0, |x|{})
    ];
    println!("{}", processor);
}
