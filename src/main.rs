use std::fs::File;
use std::io::Read;
use std::path::Path;

mod hardware;
use hardware::video::screen::SCREEN_DIMS;

use piston_window::*;

#[macro_use] extern crate text_io;
extern crate piston_window;
extern crate rand;

fn main() {
    let boot_buf = read_bin("assets/BIOS.gb");
    let rom_buf = read_bin("assets/Tetris (World).gb");
    let cartridge = hardware::cartridge::Cartridge::new(&rom_buf);    
    println!("Game data\n==========\n{}", &cartridge);

    let mut window = init_window();

    let bus = hardware::memory::bus::BUS::new(boot_buf, cartridge);
    let mut processor : hardware::cpu::CPU = hardware::cpu::CPU::new(bus);
    let mut screen = hardware::video::screen::Screen::new(&mut window);

    while let Some(e) = window.next() {
        match e {
            Event::Render(_) => {
                screen.update(&mut window, e);

                screen.set_pixel(rand::random::<u8>() % 160, rand::random::<u8>() % 140, 224, 51, 224);
            }
            _ => {}
        }

        // Here we CPU.run_frame() or something
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
    let mut window: PistonWindow = WindowSettings::new("RustikGB", 
        (hardware::video::screen::SCREEN_DIMS[0] * 2, 
         hardware::video::screen::SCREEN_DIMS[1] * 2))
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    window.set_ups(60);
    window
}