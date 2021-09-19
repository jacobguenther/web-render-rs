// File: src/config/engine_config.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct EngineConfig {
	pub canvas_id: String,
	pub width: u32,
	pub height: u32,
}
