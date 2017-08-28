use hardware::video::gpu_constants::*;
use hardware::memory::memory_region::MemoryRegion;
use std::fmt::Display;
use hardware::registers::Register;

pub type Tile = [[Register<u8>; 2]; 8];

pub struct TileSet {
    pub tiles: [Tile; TILE_NUMBER]
}

impl TileSet {
    pub fn new() -> Self {
        TileSet {
            tiles: [[[Register::new(0); 2]; 8]; TILE_NUMBER]
        }
    }

    pub fn get_pixel(&self, tile: &Tile, line: u8, pixel: u8) -> u8 {
        let hibit = if tile[line as usize][0].is_bit_set(pixel) {2} else {0} ;
        let lobit = if tile[line as usize][1].is_bit_set(pixel) {1} else {0} ;
        let color = hibit + lobit;
        color
    }

    pub fn get_tile(&self, addr: u16) -> Tile {
        self.tiles[((addr - self.start()) / 16) as usize]
    }

    pub fn dump_tiles(&self) {
        use image::{ImageBuffer, RgbaImage, Rgba};

        static HORIZONTAL_TILES: u32 = 16;
        static VERTICAL_TILES: u32 = 24;

        let mut img: RgbaImage = ImageBuffer::new(HORIZONTAL_TILES * 8, VERTICAL_TILES * 8);

        for tiley in 0..VERTICAL_TILES as usize {
            for tilex in 0..HORIZONTAL_TILES as usize {                
                let tile = self.tiles[(tiley * HORIZONTAL_TILES as usize) + tilex];
                println!("Printing tile {:?}",  tile);

                let mut debug_buffer: [[u8; 8]; 8] = [[0; 8]; 8];
                for line in 0..8 {
                    for pixel in 0..8 {
                        let color = self.get_pixel(&tile, line, pixel);
                       //println!("Setting pixel: ({}, {}), color: {}", line, pixel, color);
                        debug_buffer[line as usize][pixel as usize] = color;

                        let r = PALETTE_PINKU[color as usize][0];
                        let g = PALETTE_PINKU[color as usize][1];
                        let b = PALETTE_PINKU[color as usize][2];
                        let a = PALETTE_PINKU[color as usize][3];

                        img.put_pixel(((tilex * 8) + pixel as usize) as u32, ((tiley * 8) + line as usize) as u32, Rgba { data: [r, g, b, a]})
                    }
                }
                println!("Tile Render:");
                for i in 0..8 {
                    println!("{:?}", debug_buffer[i]);           
                }
            }
        }

        img.save("logs/tile_dump.png").unwrap();
    }
}

impl MemoryRegion for TileSet {
    fn read_byte(&self, addr: u16) -> u8 {
        let base_addr = addr - self.start();
        let tile_index = (base_addr / 16) as usize;
        let tile_row = (base_addr % 16) as usize;
        self.tiles[tile_index][tile_row / 2][tile_row % 2].r()
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        let base_addr = addr - self.start();
        let tile_index = (base_addr / 16) as usize;
        let tile_row = (base_addr % 16) as usize;
        self.tiles[tile_index][tile_row / 2][tile_row % 2].w(val);
    }

    fn in_region(&self, addr: u16) -> bool {
        addr >= self.start() && addr <= self.end()
    }
    fn start(&self) -> u16 {
        TILE_DATA_START
    }
    fn end(&self) -> u16 {
        TILE_DATA_END
    }
}
