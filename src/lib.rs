#[cfg(feature = "random")]
mod random;
#[cfg(feature = "random")]
pub use random::Random;

#[cfg(feature = "tinystr")]
pub extern crate tinystr;

#[cfg(feature = "smallvec")]
pub extern crate smallvec;

#[cfg(feature = "hash")]
pub mod hash {
	pub use ahash::{
        AHashMap as HashMap,
        AHashSet as HashSet,
        AHasher as Hasher,
    };
}