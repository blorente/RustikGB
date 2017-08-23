use std::fs::File;
use std::io::Read;
use std::path::Path;
mod cpu;

fn main() {
    let path = Path::new("assets/Tetris (World).gb");
    let rom_buf = read_bin(path);
    let mut processor : cpu::cpu::CPU = Default::default();
    let cartridge = cpu::cartridge::Cartridge::new(&rom_buf);
    println!("Game data\n==========\n{}", cartridge);
    processor.run(&cartridge);
}


fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}