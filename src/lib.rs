pub mod text;

mod timer;

mod positions;

mod bounding_box;

pub mod battle;

pub use tinystr::TinyStr16;

pub use timer::Timer;

pub use positions::direction::Direction;

pub use positions::coordinate::Coordinate;
pub use positions::pixel_offset::PixelOffset;

pub use positions::position::Position;
pub use positions::global_position::GlobalPosition;
pub use positions::location::Location;

pub use positions::destination::Destination;

pub use bounding_box::BoundingBox;

pub const WIDTH: f32 = 240.0;
pub const HEIGHT: f32 = 160.0;
pub const TILE_SIZE: f32 = 16.0;

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