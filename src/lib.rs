pub mod text;

mod timer;

mod positions;

mod bounding_box;

pub mod battle;

pub use timer::Timer;

pub use positions::direction::Direction;

pub use positions::coordinate::Coordinate;
pub use positions::pixel_offset::PixelOffset;

pub use positions::position::Position;
pub use positions::global_position::GlobalPosition;
pub use positions::location::Location;

pub use positions::destination::Destination;

pub use bounding_box::BoundingBox;

pub const TILE_SIZE: u8 = 16;

pub trait Entity {
	
	fn spawn(&mut self);
	
	fn despawn(&mut self);
	
	fn is_alive(&self) -> bool;
	
}

pub trait Reset {

	fn reset(&mut self);

}

pub trait Completable: Reset {

    fn is_finished(&self) -> bool;

}