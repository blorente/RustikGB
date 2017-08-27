use hardware::video::gpu_constants::*;
use hardware::memory::memory_region::MemoryRegion;
use std::fmt::Display;
use hardware::registers::Register;


const PALETTE_PUKE_GREEN: [[u8; 4]; 4] = [
    [157, 188, 7, 255],
    [122, 156, 107, 255],
    [ 53,  99, 56, 255],
    [ 13,  58, 8, 255],
];

type Tile = [[Register<u8>; 2]; 8];
pub struct TileSet {
    tiles: [Tile; TILE_NUMBER]
}

impl TileSet {
    pub fn new() -> Self {
        TileSet {
            tiles: [[[Register::new(0); 2]; 8]; TILE_NUMBER]
        }
    }

    pub fn dump_tiles(&self) {
        use image::{ImageBuffer, RgbaImage, Rgba};

        static HORIZONTAL_TILES: u32 = 16;
        static VERTICAL_TILES: u32 = 24;

        let mut img: RgbaImage = ImageBuffer::new(HORIZONTAL_TILES * 8, VERTICAL_TILES * 8);

        for tiley in 0..VERTICAL_TILES as usize {
            for tilex in 0..HORIZONTAL_TILES as usize {                
                let tile = self.tiles[(tiley * HORIZONTAL_TILES as usize) + tilex];
                println!("Printing tile ({}, {}): {:?}", tilex, tiley, tile);

                for line in 0..8 {
                    for pixel in 0..8 {
                        let hibit = if tile[line][0].is_bit_set(pixel) {2} else {0} ;
                        let lobit = if tile[line][1].is_bit_set(pixel) {1} else {0} ;
                        let color = hibit + lobit;

                        let r = PALETTE_PUKE_GREEN[color as usize][0];
                        let g = PALETTE_PUKE_GREEN[color as usize][1];
                        let b = PALETTE_PUKE_GREEN[color as usize][2];

                        img.put_pixel((tilex + pixel as usize) as u32, (tiley + line) as u32, Rgba { data: [r, g, b, 255]})
                    }
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
        println!("Written to TileData: {:4X}: {:2X}", addr, val);
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
