use std::fmt::{self, Display, Formatter, Write};
use std::ops::BitOr;

pub struct Mode {
    value: u32
}

impl Mode {
    /// finds if the mode indicates an executable file
    pub fn is_exe(&self) -> bool {
        (self.value & 0o111) != 0
    }
}

impl From<u32> for Mode {
    fn from(value: u32) -> Self {
        Self {
            value
        }
    }
}

impl BitOr for Mode {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self {
            value: self.value | other.value
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char(if (self.value & (1 << 8)) != 0 { 'r' } else { '-' })?;
        f.write_char(if (self.value & (1 << 7)) != 0 { 'w' } else { '-' })?;
        f.write_char(if (self.value & (1 << 6)) != 0 { 'x' } else { '-' })?;
        f.write_char(if (self.value & (1 << 5)) != 0 { 'r' } else { '-' })?;
        f.write_char(if (self.value & (1 << 4)) != 0 { 'w' } else { '-' })?;
        f.write_char(if (self.value & (1 << 3)) != 0 { 'x' } else { '-' })?;
        f.write_char(if (self.value & (1 << 2)) != 0 { 'r' } else { '-' })?;
        f.write_char(if (self.value & (1 << 1)) != 0 { 'w' } else { '-' })?;
        f.write_char(if (self.value & 1) != 0 { 'x' } else { '-' })?;
        Ok(())
    }
}

#[test]
fn test_some_print() {
    assert_eq!("rw-r--r--", Mode::from(0o644).to_string());
    let mu = Mode::from(0o600);
    let mo = Mode::from(0o004);
    assert_eq!("rw-------", mu.to_string());
    assert_eq!("------r--", mo.to_string());
    let muo = mu | mo;
    assert_eq!("rw----r--", muo.to_string());
}
