use std::fs::File;
use std::io::Read;
use std::path::Path;
mod hardware;

#[macro_use] extern crate text_io;

fn main() {

    let boot_buf = read_bin("assets/BIOS.gb");
    let rom_buf = read_bin("assets/Tetris (World).gb");
    let cartridge = hardware::cartridge::Cartridge::new(&rom_buf);
    println!("Game data\n==========\n{}", &cartridge);
    let bus = hardware::bus::BUS::new(boot_buf, cartridge);
    let mut processor : hardware::cpu::CPU = hardware::cpu::CPU::new(bus);
    processor.run();
}


fn read_bin(path: &'static str) -> Box<[u8]> {
    let path = Path::new(path);
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}