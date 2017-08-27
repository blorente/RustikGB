use std::fs::File;
use std::io::Read;
use std::path::Path;
mod hardware;

use piston_window::*;
use texture::*;

#[macro_use] extern crate text_io;
extern crate piston_window;

const SCREEN_DIMS: [u32; 2] = [160, 144];

fn main() {
    let boot_buf = read_bin("assets/BIOS.gb");
    let rom_buf = read_bin("assets/Tetris (World).gb");
    let cartridge = hardware::cartridge::Cartridge::new(&rom_buf);    
    println!("Game data\n==========\n{}", &cartridge);

    let mut window = init_window();

    let bus = hardware::memory::bus::BUS::new(boot_buf, cartridge);
    let mut processor : hardware::cpu::CPU = hardware::cpu::CPU::new(bus);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });
    }
    processor.run();
}


fn read_bin(path: &'static str) -> Box<[u8]> {
    let path = Path::new(path);
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}

fn init_window() -> PistonWindow {
    let mut window: PistonWindow = WindowSettings::new("RustikGB", (SCREEN_DIMS[0] * 2, SCREEN_DIMS[1] * 2))
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    window
}