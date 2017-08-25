
#[derive(Default)]
pub struct Register<T: Copy> {
    val: T,
}

impl<T: Copy> Register<T> {
    pub fn new(value: T) -> Self {
        Register {
            val: value
        }
    }

    pub fn r(&self) -> T {
        let ret = self.val;
        ret
    }

    pub fn w(& mut self, data: T) {
        self.val = data;
    }
}

impl Register<u8> {
    pub fn is_bit_set(&self, bit: u8) -> bool {
        return (self.val & ((1 as u8) << bit)) > 0;
    }
 }

 
impl Register<u16> {
    pub fn is_bit_set(&self, bit: u8) -> bool {
        return (self.val & ((1 as u16) << bit)) > 0;
    }
}
