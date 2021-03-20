use serde::{Deserialize, Serialize};

use crate::Coordinate;
use crate::Position;

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct GlobalPosition {

	pub local: Position,
	pub global: Coordinate,

}

impl GlobalPosition {

	pub const fn absolute(&self) -> Coordinate {
		Coordinate {
			x: self.get_x(),
			y: self.get_y()
		}
	}

	pub const fn get_x(&self) -> isize {
		self.global.x + self.local.coords.x
	}

	pub const fn get_y(&self) -> isize {
		self.global.y + self.local.coords.y
	}
 
}