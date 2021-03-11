use serde::{Serialize, Deserialize};
use glam::Vec2;

pub mod text;
mod timer;

pub use timer::Timer;

pub trait Entity {
	
	fn spawn(&mut self);
	
	fn despawn(&mut self);
	
	fn is_alive(&self) -> bool;
	
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Direction { // move to util
	
	Up,
	Down,
	Left,
	Right,
	
}

pub const DIRECTIONS: &[Direction; 4] = &[
	Direction::Up,
	Direction::Down,
	Direction::Left,
	Direction::Right,
];

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {

	pub map_id: String,
	pub map_index: u16,
	pub position: GlobalPosition,

}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct GlobalPosition {

	pub local: Position,
	pub offset: Coordinate,

}

impl GlobalPosition {

	pub fn get_x(&self) -> isize {
		self.offset.x + self.local.coords.x
	}

	pub fn get_y(&self) -> isize {
		self.offset.y + self.local.coords.y
	}
 
}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct Position {

	pub coords: Coordinate,
	pub direction: Direction,
    #[serde(skip)]
    pub offset: Vec2,

}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Coordinate {

	pub x: isize,
	pub y: isize,

}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct BoundingBox {
    pub min: Coordinate,
    pub max: Coordinate,
}

impl Direction {

	pub fn inverse(&self) -> Direction {
		match self {
		    Direction::Up => Direction::Down,
		    Direction::Down => Direction::Up,
		    Direction::Left => Direction::Right,
		    Direction::Right => Direction::Left,
		}
	}

	pub fn value(&self) -> u8 {
		match self {
			Direction::Down => 0,
			Direction::Up => 1,
			Direction::Left => 2,
			Direction::Right => 3,
		}
	}

	// Input

	pub fn offset(&self) -> (f32, f32) {
		match self {
		    Direction::Up => (0.0, -1.0),
		    Direction::Down => (0.0, 1.0),
		    Direction::Left => (-1.0, 0.0),
		    Direction::Right => (1.0, 0.0),
		}
	}

}

impl Default for Direction {
    fn default() -> Self {
        Direction::Down
    }
}

impl Coordinate {

	pub fn subtract(&self, x: isize, y: isize) -> Coordinate {
		Coordinate {
			x: self.x - x,
			y: self.y - y,
			..*self
		}
    }

	pub fn towards(&self, destination: &Coordinate) -> Direction {
		if (self.x - destination.x).abs() > (self.y - destination.y).abs() {
			if self.x > destination.x {
				Direction::Left
			} else {
				Direction::Right
			}
		} else {
			if self.y > destination.y {
				Direction::Up
			} else {
				Direction::Down
			}
		}
	}
}

impl BoundingBox {

    pub fn in_bounds(&self, coordinate: &Coordinate) -> bool{
        if coordinate.x >= self.min.x && coordinate.x <= self.max.x {
            return coordinate.y >= self.min.y && coordinate.y <= self.max.y;
        } else {
            return false;
        }
    }

}