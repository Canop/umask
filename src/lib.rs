//! A light utility helping with unix mode representation.
//!
//! (light here means there aren't many features)
//!
//! The Mode struct implements Displays and prints as `"rwxrwxrwx"`
//!
//! ```
//!    use umask::Mode;
//!
//!    assert_eq!("rw-r--r--", Mode::from(0o644).to_string());
//!    let mu = Mode::from(0o600);
//!    let mo = Mode::from(0o004);
//!    assert_eq!("rw-------", mu.to_string());
//!    assert_eq!("------r--", mo.to_string());
//!    let muo = mu | mo;
//!    assert_eq!("rw----r--", muo.to_string());
//! ```
mod mode;

pub use mode::Mode;
