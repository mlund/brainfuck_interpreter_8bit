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

impl std::ops::AddAssign<u8> for Unit {
    fn add_assign(&mut self, rhs: u8) {
        self.value += rhs as i16;
        if self.value > 256 {
            self.value -= 256;
        }
    }
}

impl std::ops::SubAssign<u8> for Unit {
    fn sub_assign(&mut self, rhs: u8) {
        self.value -= rhs as i16;

        if self.value < 0 {
            self.value += 256;
        }
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plus_minus_symbol() {
        let mut a = Unit::new(8);
        a += 3;
        assert_eq!(11, a.get_raw());
        a -= 3;
        assert_eq!(8, a.get_raw());
    }
    #[test]
    fn test_wrap_around() {
        let mut a = Unit::new(250);
        a += 7;
        assert_eq!(1, a.get_raw());

        a -= 7;
        assert_eq!(250, a.get_raw());
    }
    #[test]
    fn test_acsii_char() {
        let a = Unit::new(50);
        let b = a.get_char();
        assert_eq!(b, '2');
    }
}
