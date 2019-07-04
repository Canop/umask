[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/umask.svg
[l1]: https://crates.io/crates/umask

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/umask/badge.svg
[l3]: https://docs.rs/umask/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3

# umask

An utility to build and display unix access permission modes

### Import

In Cargo.toml:

    umask = "0.1"


### Usage

 ```
     use umask::*;

     // You can build from a number:
     assert_eq!("rw-r--r--", Mode::from(0b110100100).to_string());
     assert_eq!("rw-r--r--", Mode::from(0o644).to_string());

     // You may use `|` to combine class permissions:
     let mu = Mode::from(0o600);
     let mo = Mode::from(0o004);
     let muo = mu | mo;
     assert_eq!("rw----r--", muo.to_string());

     // You can build with semantic constructs:
     let m = Mode::all()
         .without(ALL_EXEC);
     assert_eq!("rw-rw-rw-", m.to_string());
     let mut m = Mode::new()
         .with_class_perm(ALL, READ)
         .with_class_perm(USER, WRITE);
     assert_eq!("rw-r--r--", m.to_string());

     // Or if you like
     m |= ALL_EXEC;
     assert_eq!("rwxr-xr-x", m.to_string());
     let m = ALL_READ | USER_WRITE;
     assert_eq!("rw-r--r--", m.to_string());

 ```
