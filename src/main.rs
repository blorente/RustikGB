use std::fs::File;
use std::io::Read;
use std::path::Path;
mod hardware;

fn main() {
    let path = Path::new("assets/Tetris (World).gb");
    let rom_buf = read_bin(path);
    let cartridge = hardware::cartridge::Cartridge::new(&rom_buf);
    println!("Game data\n==========\n{}", &cartridge);
    let bus = hardware::bus::BUS::new(cartridge);
    let mut processor : hardware::cpu::CPU = hardware::cpu::CPU::new(bus);
    processor.run();
}


fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}