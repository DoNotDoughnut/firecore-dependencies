pub extern crate serde;

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

#[cfg(feature = "util")]
pub mod text;

#[cfg(feature = "util")]
mod timer;

#[cfg(feature = "util")]
mod positions;

#[cfg(feature = "util")]
mod bounding_box;

#[cfg(feature = "util")]
pub mod battle;

#[cfg(feature = "util")]
pub use {
    timer::Timer,
    positions::{
        direction::Direction,
        coordinate::Coordinate,
        pixel_offset::PixelOffset,
        position::Position,
        global_position::GlobalPosition,
        destination::Destination,
    },
    bounding_box::BoundingBox
};

#[cfg(all(feature = "tinystr", feature = "util"))]
pub use positions::location::Location;

#[cfg(feature = "util")]
pub const WIDTH: f32 = 240.0;
#[cfg(feature = "util")]
pub const HEIGHT: f32 = 160.0;
#[cfg(feature = "util")]
pub const TILE_SIZE: f32 = 16.0;

#[cfg(feature = "util")]
pub trait Entity {
	
	fn spawn(&mut self);
	
	fn despawn(&mut self);
	
	fn is_alive(&self) -> bool;
	
}

#[cfg(feature = "util")]
pub trait Reset {

	fn reset(&mut self);

}

#[cfg(feature = "util")]
pub trait Completable: Reset {

    fn is_finished(&self) -> bool;

}