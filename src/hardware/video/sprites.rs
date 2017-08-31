use hardware::memory::memory_region::MemoryRegion;
use hardware::registers::Register;

pub const SPRITE_OAM_START              : u16 = 0xFE00;
pub const SPRITE_OAM_END                : u16 = 0xFE9F;

#[derive(Copy, Clone)]
pub struct Sprite {
    pub coord_y: u8,
    pub coord_x: u8,
    pub data_tile: u8,
    pub priority: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub palette_0: bool,
}

impl Sprite {
    fn new() -> Self {
        Sprite {
            coord_y: 0,
            coord_x: 0,
            data_tile: 0,
            priority: false,
            flip_x: false,
            flip_y: false,
            palette_0: false,
        }
    }
}

pub struct SpriteOAM {
    pub sprites: [Sprite; 40]
}

impl SpriteOAM {
    pub fn new() -> Self {
        SpriteOAM{
            sprites: [Sprite::new(); 40]
        }
    }

    pub fn sprite_num_from_addr(&self, addr: u16) -> u8 {
        (addr - SPRITE_OAM_START) as u8 / 4
    }

    pub fn sprite_byte_offset(&self, addr: u16) -> u8 {
        (addr - SPRITE_OAM_START) as u8 % 4
    }
}

impl MemoryRegion for SpriteOAM {
    fn read_byte(&self, addr: u16) -> u8 {
        let sprite_index = self.sprite_num_from_addr(addr) as usize;
        let sprite_offset = self.sprite_byte_offset(addr);
        match sprite_offset {
            0 => {self.sprites[sprite_index].coord_y}
            1 => {self.sprites[sprite_index].coord_x}
            2 => {self.sprites[sprite_index].data_tile}
            3 => {
                let mut res : Register<u8> = Register::new(0);
                res.set_bit(4, self.sprites[sprite_index].palette_0);
                res.set_bit(5, self.sprites[sprite_index].flip_x);
                res.set_bit(6, self.sprites[sprite_index].flip_y);
                res.set_bit(7, self.sprites[sprite_index].priority);
                res.r()
            }
            _ => {panic!("Something went wrong when reading byte {:X} from sprite {:}", sprite_offset, sprite_index);}
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        let sprite_index = self.sprite_num_from_addr(addr) as usize;
        let sprite_offset = self.sprite_byte_offset(addr);
        match sprite_offset {
            0 => {self.sprites[sprite_index].coord_y = val;}
            1 => {self.sprites[sprite_index].coord_x = val;}
            2 => {self.sprites[sprite_index].data_tile = val;}
            3 => {
                self.sprites[sprite_index].palette_0    = (val & (1 << 4)) > 0;
                self.sprites[sprite_index].flip_x       = (val & (1 << 5)) > 0;
                self.sprites[sprite_index].flip_y       = (val & (1 << 6)) > 0;
                self.sprites[sprite_index].priority     = (val & (1 << 7)) > 0;
            }
            _ => {panic!("Something went wrong when writing to byte {:X} from sprite {:}", sprite_offset, sprite_index);}
        }
    }

    fn in_region(&self, addr: u16) -> bool {
        addr >= self.start() && addr <= self.end()
    }
    fn start(&self) -> u16 {
        SPRITE_OAM_START
    }
    fn end(&self) -> u16 {
        SPRITE_OAM_END
    }
}

