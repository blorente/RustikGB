use hardware::memory::memory_region::MemoryRegion;
use hardware::memory::memory_region::BitAccess;

pub struct PLAIN_RAM {
    storage: Vec<u8>,
    start: u16,
    end: u16
}

impl PLAIN_RAM {
    pub fn new(start: u16, end: u16) -> Self {
        PLAIN_RAM {
            start: start,
            end: end,
            storage: vec![0x0; end as usize - start as usize + 1]
        }
    }

    pub fn from_data(start: u16, end: u16, data: Box<[u8]>) -> Self {
        PLAIN_RAM {
            start: start,
            end: end,
            storage: data.to_vec()
        }
    }
}

impl MemoryRegion for PLAIN_RAM {
    fn read_byte(&self, addr: u16) -> u8 {
        let tru_addr = addr - self.start();
        self.storage[tru_addr as usize]
    }


    fn write_byte(&mut self, addr: u16, val: u8) {
        let tru_addr = addr - self.start();
        self.storage[tru_addr as usize] = val;
    }

    fn in_region(&self, addr: u16) -> bool {
        addr >= self.start() && addr <= self.end()
    }

    fn start(&self) -> u16 {
        self.start
    }
    fn end(&self) -> u16 {
        self.end
    }
}

impl BitAccess for PLAIN_RAM {    
    fn read_bit(&self, addr: u16, bit: u8) -> bool {
        let val = self.read_byte(addr);
        val & (1 << bit) > 0
    }

    fn set_bit(&mut self, addr: u16, bit: u8, val: bool) {
        let cur_val = self.read_byte(addr);
        let tru_addr = addr - self.start();
        if val {
            self.storage[tru_addr as usize] = cur_val | (1 << bit);
        } else {
            self.storage[tru_addr as usize] = cur_val & !(1 << bit);
        }
    }
}