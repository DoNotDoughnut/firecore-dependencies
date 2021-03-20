use serde::{Deserialize, Serialize};

use crate::GlobalPosition;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {

	pub map_id: String,
	pub map_index: u16,
	pub position: GlobalPosition,

}