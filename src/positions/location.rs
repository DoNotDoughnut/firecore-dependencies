use serde::{Deserialize, Serialize};
use tinystr::TinyStr16;

use crate::GlobalPosition;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {

	pub map: Option<TinyStr16>,
	pub index: TinyStr16,
	pub position: GlobalPosition,

}