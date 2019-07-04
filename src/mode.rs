use std::fmt::{self, Display, Formatter, Write};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

pub type Class = u32;
pub const USER: Class = 0b111000000;
pub const GROUP: Class = 0b000111000;
pub const OTHER: Class = 0b000000111;
pub const ALL: Class = 0b111111111;

pub type Permission = u32;
pub const READ: Permission = 0b100100100;
pub const WRITE: Permission = 0b010010010;
pub const EXEC: Permission = 0b001001001;

pub const USER_READ: Mode = Mode::new().with_class_perm(USER, READ);
pub const USER_WRITE: Mode = Mode::new().with_class_perm(USER, WRITE);
pub const USER_EXEC: Mode = Mode::new().with_class_perm(USER, EXEC);
pub const GROUP_READ: Mode = Mode::new().with_class_perm(GROUP, READ);
pub const GROUP_WRITE: Mode = Mode::new().with_class_perm(GROUP, WRITE);
pub const GROUP_EXEC: Mode = Mode::new().with_class_perm(GROUP, EXEC);
pub const OTHER_READ: Mode = Mode::new().with_class_perm(OTHER, READ);
pub const OTHER_WRITE: Mode = Mode::new().with_class_perm(OTHER, WRITE);
pub const OTHER_EXEC: Mode = Mode::new().with_class_perm(OTHER, EXEC);
pub const ALL_READ: Mode = Mode::new().with_class_perm(ALL, READ);
pub const ALL_WRITE: Mode = Mode::new().with_class_perm(ALL, WRITE);
pub const ALL_EXEC: Mode = Mode::new().with_class_perm(ALL, EXEC);

#[derive(Clone, Copy)]
pub struct Mode {
    value: u32
}

impl Default for Mode {
    fn default() -> Self {
        Self {
            value: 0
        }
    }
}

impl From<u32> for Mode {
    fn from(value: u32) -> Self {
        Self {
            value
        }
    }
}

impl BitAnd for Mode {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self {
            value: self.value & other.value
        }
    }
}

impl BitAndAssign for Mode {
    fn bitand_assign(&mut self, other: Self) {
        self.value &= other.value;
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

impl BitOrAssign for Mode {
    fn bitor_assign(&mut self, other: Self) {
        self.value |= other.value;
    }
}

impl Not for Mode {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self {
            value: ! self.value
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

impl Mode {
    /// build a mode with absolutely no permission
    pub const fn new() -> Self {
        Self {
            value: 0
        }
    }
    /// build a mode with all permissions given to everybody
    pub const fn all() -> Self {
        Self {
            value: 0b111111111
        }
    }
    /// finds if the mode indicates an executable file
    #[inline(always)]
    pub const fn is_exe(&self) -> bool {
        (self.value & 0o111) != 0
    }
    /// return a new mode, with the permission added for the class
    /// (does nothing if the permission is already given to that class)
    #[inline(always)]
    pub const fn with_class_perm(self, class: Class, perm: Permission) -> Self {
        Self {
            value: self.value | ( class & perm )
        }
    }
    /// return a new mode, with the permission removed for the class
    /// (does nothing if the permission is already given to that class)
    #[inline(always)]
    pub const fn without_class_perm(self, class: Class, perm: Permission) -> Self {
        Self {
            value: self.value & !( class & perm )
        }
    }
    /// add the class/permissions of the other mode
    #[inline(always)]
    pub const fn with(self, other: Mode) -> Self {
        Self {
            value: self.value | other.value
        }
    }
    /// remove the class/permissions of the other mode
    #[inline(always)]
    pub const fn without(self, other: Mode) -> Self {
        Self {
            value: self.value & ! other.value
        }
    }
}

#[test]
fn test_build() {
    let mut m = Mode::new()
        .with_class_perm(ALL, READ)
        .with_class_perm(USER, WRITE);
    assert_eq!("rw-r--r--", m.to_string());
    m |= ALL_EXEC;
    assert_eq!("rwxr-xr-x", m.to_string());
    m &= !USER_READ;
    assert_eq!("-wxr-xr-x", m.to_string());
}

#[test]
fn test_print() {
    assert_eq!("rw-r--r--", Mode::from(0b110100100).to_string());
    assert_eq!("rw-r--r--", Mode::from(0o644).to_string());
    let mu = Mode::from(0o600);
    let mo = Mode::from(0o004);
    assert_eq!("rw-------", mu.to_string());
    assert_eq!("------r--", mo.to_string());
    let muo = mu | mo;
    assert_eq!("rw----r--", muo.to_string());
}
