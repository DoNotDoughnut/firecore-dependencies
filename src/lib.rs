#[cfg(feature = "random")]
pub mod random;

#[cfg(feature = "borrow")]
pub mod borrow;

#[cfg(feature = "str")]
pub extern crate tinystr as str;

#[cfg(feature = "vec")]
pub extern crate arrayvec as vec;

#[cfg(feature = "hash")]
pub extern crate hashbrown as hash;

#[cfg(feature = "ser")]
pub mod ser {

    extern crate bincode;

    pub use bincode::{deserialize, serialize, Error, ErrorKind};
}
