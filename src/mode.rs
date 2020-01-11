use std::fmt::{self, Display, Formatter, Write};
use std::io;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};
use std::path::Path;

#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

pub type Class = u32;

pub const USER: Class = 0b111000000;
pub const GROUP: Class = 0b000111000;
pub const OTHERS: Class = 0b000000111;
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
pub const OTHERS_READ: Mode = Mode::new().with_class_perm(OTHERS, READ);
pub const OTHERS_WRITE: Mode = Mode::new().with_class_perm(OTHERS, WRITE);
pub const OTHERS_EXEC: Mode = Mode::new().with_class_perm(OTHERS, EXEC);
pub const ALL_READ: Mode = Mode::new().with_class_perm(ALL, READ);
pub const ALL_WRITE: Mode = Mode::new().with_class_perm(ALL, WRITE);
pub const ALL_EXEC: Mode = Mode::new().with_class_perm(ALL, EXEC);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mode {
    value: u32,
}

impl Default for Mode {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<u32> for Mode {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl BitAnd for Mode {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self {
            value: self.value & other.value,
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
            value: self.value | other.value,
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
        Self { value: !self.value }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char(if self.has(USER_READ) { 'r' } else { '-' })?;
        f.write_char(if self.has(USER_WRITE) { 'w' } else { '-' })?;
        f.write_char(if self.has(USER_EXEC) { 'x' } else { '-' })?;
        f.write_char(if self.has(GROUP_READ) { 'r' } else { '-' })?;
        f.write_char(if self.has(GROUP_WRITE) { 'w' } else { '-' })?;
        f.write_char(if self.has(GROUP_EXEC) { 'x' } else { '-' })?;
        f.write_char(if self.has(OTHERS_READ) { 'r' } else { '-' })?;
        f.write_char(if self.has(OTHERS_WRITE) { 'w' } else { '-' })?;
        f.write_char(if self.has(OTHERS_EXEC) { 'x' } else { '-' })?;
        Ok(())
    }
}

impl Mode {
    /// build a mode with absolutely no permission
    #[inline(always)]
    pub const fn new() -> Self {
        Self { value: 0 }
    }
    /// build a mode with all permissions given to everybody
    #[inline(always)]
    pub const fn all() -> Self {
        Self { value: 0b111111111 }
    }
    /// return the mode for the given path.
    /// On non unix platforms, return `Mode::all()`
    #[allow(unused_variables)]
    pub fn try_from(path: &Path) -> Result<Self, io::Error> {
        #[cfg(unix)]
        {
            let metadata = fs::metadata(&path)?;
            Ok(Mode::from(metadata.mode()))
        }
        #[cfg(not(unix))]
        Ok(Self::all())
    }
    /// finds if the mode indicates an executable file
    #[inline(always)]
    pub const fn is_exe(self) -> bool {
        (self.value & 0o111) != 0
    }
    /// indicates whether the passed class/permissions are all present in self
    #[inline(always)]
    pub const fn has(self, other: Self) -> bool {
        self.value & other.value == other.value
    }
    /// return a new mode, with the permission added for the class
    /// (does nothing if the permission is already given to that class)
    #[inline(always)]
    pub const fn with_class_perm(self, class: Class, perm: Permission) -> Self {
        Self {
            value: self.value | (class & perm),
        }
    }
    /// return a new mode, with the permission removed for the class
    /// (does nothing if the permission is already given to that class)
    #[inline(always)]
    pub const fn without_class_perm(self, class: Class, perm: Permission) -> Self {
        Self {
            value: self.value & !(class & perm),
        }
    }
    /// add the class/permissions of the other mode
    #[inline(always)]
    pub const fn with(self, other: Mode) -> Self {
        Self {
            value: self.value | other.value,
        }
    }
    /// remove the class/permissions of the other mode
    #[inline(always)]
    pub const fn without(self, other: Mode) -> Self {
        Self {
            value: self.value & !other.value,
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
