#[derive(Debug)]
pub struct Unit {
    value: i16,
}

impl Unit {
    /// Create new Unit from u8
    pub fn new(v: u8) -> Self {
        Unit { value: v as i16 }
    }
    /// Create new Unit from char
    pub fn new_from_char(v: &char) -> Self {
        let b = v.clone() as u8;

        Unit { value: b as i16 }
    }
    /// Get u8 value of the Unit
    pub fn get_raw(&self) -> u8 {
        self.value as u8
    }
    /// Get ASCII char from Unit
    pub fn get_char(&self) -> char {
        char::from_u32(self.value as u32).unwrap()
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
