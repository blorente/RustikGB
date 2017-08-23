use std::fmt;
use std::str;

pub enum CartridgeType {
    ROM_ONLY, // 00
    UNKNOWN
}

impl fmt::Display for CartridgeType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ROM_ONLY => write!(fmt, "ROM_ONLY"),
            UNKNOWN => write!(fmt, "UNRECOGNIZED")
        }
    }
}

pub struct CartridgeHeader {

    // Addr: 0x0100-0x0103
    // Usually NOP and JP
    entry_point: [u16; 2],

    // Addr: 0x0104-0x0133
    // Just raw data
    nintendo_graphic: [u8; 47],

    // Addr: 0x0134-0x0142
    // Standard UPPERCASE ASCII 
    // with trailing spaces
    game_title: String,

    // Addr: 0x0147
    // 00 -> ROM Only (Tetris)
    // ...
    cartridge_type: CartridgeType,
}

impl CartridgeHeader {
    pub fn new(raw_rom: &Box<[u8]>) -> Self {
        CartridgeHeader {
            entry_point: CartridgeHeader::read_entry_point(raw_rom),
            nintendo_graphic: CartridgeHeader::read_nintendo_graphic(raw_rom),
            cartridge_type: CartridgeHeader::read_cartridge_type(raw_rom),
            game_title:  CartridgeHeader::read_game_title(raw_rom),
        }
    }

    fn read_entry_point(raw_rom: &Box<[u8]>) -> [u16; 2] {        
        [raw_rom[0x0100] as u16, raw_rom[0x0101] as u16]
    }

    fn read_nintendo_graphic(raw_rom: &Box<[u8]>) -> [u8; 47] {
        let mut graphic : [u8; 47] = [0; 47];
        graphic.copy_from_slice(&raw_rom[0x0104..0x0133]);
        graphic
    }

    fn read_game_title(raw_rom: &Box<[u8]>) -> String {
        let mut decoded = String::from(str::from_utf8(&(raw_rom[0x0134..0x0142])).unwrap());
        while decoded.ends_with('\0') {
            let len = decoded.len();
            let new_len = len.saturating_sub(1);
            decoded.truncate(new_len);
        }
        decoded
    }

    fn read_cartridge_type(raw_rom: &Box<[u8]>) -> CartridgeType {
        match raw_rom[0x0147] {
            0x00 => CartridgeType::ROM_ONLY,
            _ => CartridgeType::UNKNOWN
        }
    }
}

impl fmt::Display for CartridgeHeader {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "Game Title: {:?}\nROM Type: {}\n", 
                self.game_title,
                self.cartridge_type)
    }
}

pub struct Cartridge {
    header: CartridgeHeader,
    data: Box<[u8]>
}

impl Cartridge {
    pub fn new(raw_rom: &Box<[u8]>) -> Self {
        Cartridge {
            header: CartridgeHeader::new(raw_rom),
            data: Cartridge::copy_raw_rom(raw_rom)
        }
    }

    fn copy_raw_rom(raw_rom: &Box<[u8]>) -> Box<[u8]> {
        raw_rom.clone()
    }

    /// Read the byte in addr
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    /// Read the bytes in [addr, addr + 1]
    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = (self.data[addr as usize] as u16);
        let hi = (self.data[(addr + 1) as usize] as u16) << 8;
        (hi | lo)
    }
}

impl fmt::Display for Cartridge {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "{}", self.header)
    }
}