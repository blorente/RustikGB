pub trait MemoryRegion {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, val: u8);

    fn in_region(&self, addr: u16) -> bool;
    fn start(&self) -> u16;
    fn end(&self) -> u16;
}

pub trait BitAccess {
    fn read_bit(&self, addr: u16, bit: u8) -> bool;
    fn set_bit(&mut self, addr: u16, bit: u8, val: bool);
}