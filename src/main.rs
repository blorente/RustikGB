mod cpu;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    let file = File::open(&Path::new("../assets/Tetris (World).gb");
    let mut rom_buf = [0u8; 12];
    let bytes_read = file.read(&mut rom_buf).unwrap();
    if bytes_read != rom_buf.len() {
        println!("{} bytes read, but {} expected ...", bytes_read, rom_buf.len());
        // handle error or bail out
    }
    let processor : cpu::cpu::CPU = Default::default();

    println!("{}", processor);
}

fn read_bin<P: >(path: P) -> 
