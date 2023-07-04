#[derive(Debug, Default, Clone, Copy)]
pub struct Unit {
    value: i16,
}

impl From<Unit> for char {
    fn from(value: Unit) -> Self {
        char::from_u32(u8::from(value) as u32).unwrap()
    }
}

impl From<Unit> for u8 {
    fn from(value: Unit) -> Self {
        value.value as u8
    }    
}

impl From<char> for Unit {
    fn from(value: char) -> Self {
        Unit { value: value as i16 }
    }
}

impl core::ops::AddAssign<u8> for Unit {
    fn add_assign(&mut self, rhs: u8) {
        self.value += rhs as i16;
        if self.value > 256 {
            self.value -= 256;
        }
    }
}

impl core::ops::SubAssign<u8> for Unit {
    fn sub_assign(&mut self, rhs: u8) {
        self.value -= rhs as i16;
        if self.value < 0 {
            self.value += 256;
        }
    }
}

impl core::fmt::Display for Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value)
    }
}