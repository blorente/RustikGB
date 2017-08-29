use hardware::memory::memory_region::MemoryRegion;
use hardware::registers::Register;

const INTERRUPT_ENABLE_ADDR     : u16 = 0xFFFF;
const INTERRUPT_FLAG_ADDR       : u16 = 0xFF0F;

pub const VBLANK_ISR_START      : u16 = 0x0040;
pub const LCDC_ISR_START        : u16 = 0x0048;
pub const TIMER_ISR_START       : u16 = 0x0050;
pub const SERIAL_ISR_START      : u16 = 0x0058;
pub const JOYPAD_ISR_START      : u16 = 0x0060;

#[derive(Debug)]
pub enum InterruptType {
    Pad         = 4,
    Serial      = 3,
    Timer       = 2,
    LCDC        = 1,
    VBlank      = 0
}
pub struct Interrupts {
    are_enabled: bool,
    interrupt_enable: Register<u8>,
    interrupt_flags: Register<u8>,

    // Handlers for delay-changing the state.
    // cycles_before_change == -1 means it shouldn't change
    steps_before_change: i8,
    target_enable_state: bool
}

impl Interrupts {
    pub fn new() -> Self {
        Interrupts {
            are_enabled: false,
            interrupt_enable: Register::new(0),
            interrupt_flags: Register::new(0),

            steps_before_change: -1,
            target_enable_state: false
        }
    }

    pub fn are_enabled(&self) -> bool {
        self.are_enabled
    }

    pub fn read_and_clear(&mut self, interrupt: InterruptType) -> bool {
        let it = interrupt as u8;
        let is_set = self.interrupt_flags.is_bit_set(it);
        self.interrupt_flags.set_bit(it, false);
        is_set
    }

    pub fn set_interrupt(&mut self, interrupt: InterruptType) {
        let it = interrupt as u8;
        if self.are_enabled && self.interrupt_enable.is_bit_set(it) {
            println!("Interrupt set: {:?}", it);
            self.interrupt_flags.set_bit(it, true);            
        }
    }

    pub fn step(&mut self, cycles: u32) {
        if self.steps_before_change > 0 {
            self.steps_before_change -= 1;
        } else if self.steps_before_change == 0 {
            self.are_enabled = self.target_enable_state;
            self.steps_before_change = -1;
        }
    }

    pub fn disable(&mut self) {
        self.are_enabled = false;
        self.steps_before_change = -1;
    }

    pub fn enable(&mut self) {
        self.are_enabled = true;
        self.steps_before_change = -1;
    }

    pub fn disable_in_next_step(&mut self) {
        self.steps_before_change = 1;
        self.target_enable_state = false;
    }

    pub fn enable_in_next_step(&mut self) {
        self.steps_before_change = 1;
        self.target_enable_state = true;
    }
}

impl MemoryRegion for Interrupts {
    fn read_byte(&self, addr: u16) -> u8 {
        if addr == INTERRUPT_ENABLE_ADDR {
            self.interrupt_enable.r()
        } else if addr == INTERRUPT_FLAG_ADDR {
            self.interrupt_flags.r()
        } else {
            panic!("Trying to read a wrong address ({:4X}) from Interrupts", addr);
        }
    }
    fn write_byte(&mut self, addr: u16, val: u8) {
        if addr == INTERRUPT_ENABLE_ADDR {
            self.interrupt_enable.w(val)
        } else if addr == INTERRUPT_FLAG_ADDR {
            self.interrupt_flags.w(val)
        } else {
            panic!("Trying to write {:2X} to a wrong address ({:4X}) from Interrupts", val, addr);
        }
    }

    fn in_region(&self, addr: u16) -> bool {
        addr == INTERRUPT_ENABLE_ADDR
        || addr == INTERRUPT_FLAG_ADDR
    }
    fn start(&self) -> u16{
        panic!("Interrupts don't have just one start")
    }
    fn end(&self) -> u16{
        panic!("Interrupts don't have just one end")
    }
}