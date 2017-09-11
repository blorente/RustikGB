use piston_window::*;
use std::fmt;
use std::fmt::Display;

use hardware::memory::memory_region::MemoryRegion;
use hardware::registers::Register;
use hardware::interrupts::Interrupts;
use hardware::interrupts::InterruptType;

pub const JOYPAD_ADDR : u16 = 0xFF00;

const B_SELECT_BUTTON    : u8 = 5;
const B_SELECT_DIRECTION : u8 = 4;
const B_DOWN_OR_START    : u8 = 3;
const B_UP_OR_SELECT     : u8 = 2;
const B_LEFT_OR_B        : u8 = 1;
const B_RIGHT_OR_A       : u8 = 0;

enum Button {A, B, Up, Down, Left, Right, Start, Select}

pub struct Joypad {
    state: Register<u8>,
    pressed_a: bool,
    pressed_b: bool,
    pressed_up: bool,
    pressed_down: bool,
    pressed_left: bool,
    pressed_right: bool,
    pressed_select: bool,
    pressed_start: bool,

    interrupt_generated: bool
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            state: Register::new(0),
            pressed_a:      false,
            pressed_b:      false,
            pressed_up:     false,
            pressed_down:   false,
            pressed_left:   false,
            pressed_right:  false,
            pressed_select: false,
            pressed_start:  false,

            interrupt_generated: false
        }
    }

    pub fn process_press(&mut self, key: Key) {
        match key {
            Key::Z      => {self.set_key_press(Button::A, true)}
            Key::X      => {self.set_key_press(Button::B, true)}
            Key::Up     => {self.set_key_press(Button::Up, true)}
            Key::Down   => {self.set_key_press(Button::Down, true)}
            Key::Left   => {self.set_key_press(Button::Left, true)}
            Key::Right  => {self.set_key_press(Button::Right, true)}
            Key::N      => {self.set_key_press(Button::Start, true)}
            Key::M      => {self.set_key_press(Button::Select, true)}
            _           => {}
        }

        println!("{}", self);
    }

     pub fn process_release(&mut self, key: Key) {
        match key {
            Key::Z      => {self.set_key_press(Button::A, false)}
            Key::X      => {self.set_key_press(Button::B, false)}
            Key::Up     => {self.set_key_press(Button::Up, false)}
            Key::Down   => {self.set_key_press(Button::Down, false)}
            Key::Left   => {self.set_key_press(Button::Left, false)}
            Key::Right  => {self.set_key_press(Button::Right, false)}
            Key::N      => {self.set_key_press(Button::Start, false)}
            Key::M      => {self.set_key_press(Button::Select, false)}
            _           => {}
        }
    }

    pub fn step(&mut self, cycles: u32, interrupt_handler: &mut Interrupts) {
        if self.interrupt_generated {
            interrupt_handler.set_interrupt(InterruptType::Pad);
            self.interrupt_generated = false;
        }
    }

    fn set_key_press(&mut self, button: Button, on: bool) {
        match button {
            Button::A       => {self.pressed_a = on;} 
            Button::B       => {self.pressed_b = on;} 
            Button::Up      => {self.pressed_up = on;} 
            Button::Down    => {self.pressed_down = on;} 
            Button::Left    => {self.pressed_left = on;} 
            Button::Right   => {self.pressed_right = on;} 
            Button::Start   => {self.pressed_start = on;} 
            Button::Select  => {self.pressed_select = on;}
        }

        if on {
            self.interrupt_generated = true;
        }
    }

    fn is_reading_direction(&self) -> bool {
        self.state.r() & (1 << B_SELECT_DIRECTION) == 0
    } 

    fn is_reading_buttons(&self) -> bool {
        self.state.r() & (1 << B_SELECT_BUTTON) == 0
    }
}

impl Display for Joypad {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "Joypad state:");
        writeln!(fmt, "A:      {} | Right: {}", self.pressed_a, self.pressed_right);
        writeln!(fmt, "B:      {} | Left:  {}", self.pressed_b, self.pressed_left);
        writeln!(fmt, "Sel:    {} | Up:    {}", self.pressed_select, self.pressed_up);
        writeln!(fmt, "Sta:    {} | Down:  {}", self.pressed_start, self.pressed_down);
        writeln!(fmt, "Read byte: {:8b}", self.read_byte(JOYPAD_ADDR))
    }
}

impl MemoryRegion for Joypad {
    fn read_byte(&self, addr: u16) -> u8 {
        let mut ret = Register::new(self.state.r());
        if  self.is_reading_direction() {
            ret.set_bit(B_RIGHT_OR_A,       !self.pressed_right);
            ret.set_bit(B_LEFT_OR_B,        !self.pressed_left);
            ret.set_bit(B_DOWN_OR_START,    !self.pressed_down);
            ret.set_bit(B_UP_OR_SELECT,     !self.pressed_up);
        } else if self.is_reading_buttons() {
            ret.set_bit(B_RIGHT_OR_A,       !self.pressed_a);
            ret.set_bit(B_LEFT_OR_B,        !self.pressed_b);
            ret.set_bit(B_DOWN_OR_START,    !self.pressed_start);
            ret.set_bit(B_UP_OR_SELECT,     !self.pressed_select);
        }
        ret.r()
    }
    fn write_byte(&mut self, addr: u16, val: u8) {
        if val & (1 << B_SELECT_BUTTON) == 0
            && val & (1 << B_SELECT_DIRECTION) == 0 {
                panic!("Trying to read both Buttons and Direction at the same time!!");
        }
        self.state.set_bit(B_SELECT_BUTTON,     val & (1 << B_SELECT_BUTTON) > 0);        
        self.state.set_bit(B_SELECT_DIRECTION,  val & (1 << B_SELECT_DIRECTION) > 0);
    }

    fn in_region(&self, addr: u16) -> bool {
        addr == JOYPAD_ADDR
    }
    fn start(&self) -> u16 {
        JOYPAD_ADDR
    }
    fn end(&self) -> u16 {
        JOYPAD_ADDR
    }
}