macro_rules! quick_fix {
    ($exec: block, $panic: expr) => {{
        const QUICK_FIXES_ACTIVE : bool = true;
        if (QUICK_FIXES_ACTIVE) {
            $exec
        } else {
            panic!($panic);
        }
    }};
}

pub mod cpu;
pub mod cartridge;
pub mod instructions;
pub mod memory;
pub mod debugger;
pub mod registers;
pub mod video;
pub mod interrupts;
pub mod joypad;

pub fn hex_print(title: &'static str, data: &[u8], bytes_per_line: u8) {
    println!("{}", title);
    println!("================");
    print!("  |");
    for i in 0..bytes_per_line {
        print!("{:2X}|", i);
    }
    println!();
    print!("  -");
    for i in 0..bytes_per_line {
        print!("---");
    }
    let mut line_counter = bytes_per_line;
    let mut line_no = 0;
    for byte in data {
        if line_counter == bytes_per_line {
            println!();
            print!("{:2X}|", line_no);
            line_counter = 0;
            line_no += 1;
        }
        print!("{:2X} ", byte);
        line_counter += 1;
    }
    println!();
}