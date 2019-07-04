//! A light utility helping with unix mode representation, with strong
//!  types to avoid misusing constants.
//!
//! The Mode struct implements Displays and prints as `"rwxrwxrwx"`
//!
//! ```
//! use umask::*;
//!
//! // You can build from a number:
//! assert_eq!("rw-r--r--", Mode::from(0b110100100).to_string());
//! assert_eq!("rw-r--r--", Mode::from(0o644).to_string());
//!
//! // You may use `|` to combine class permissions:
//! let mu = Mode::from(0o600);
//! let mo = Mode::from(0o004);
//! let muo = mu | mo;
//! assert_eq!("rw----r--", muo.to_string());
//!
//! // You can use more semantic constructs:
//! let m = Mode::all()
//!     .without(ALL_EXEC);
//! assert_eq!("rw-rw-rw-", m.to_string());
//! let mut m = Mode::new()
//!     .with_class_perm(ALL, READ)
//!     .with_class_perm(USER, WRITE);
//! assert_eq!("rw-r--r--", m.to_string());
//! // (semantic functions can be used in const declarations)
//!
//! // Or
//! m |= ALL_EXEC;
//! assert_eq!("rwxr-xr-x", m.to_string());
//! let m = ALL_READ | USER_WRITE;
//! assert_eq!("rw-r--r--", m.to_string());
//!
//! ```
mod mode;

pub use mode::*;
