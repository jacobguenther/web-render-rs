// File: src/config/mod.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

pub mod engine_config;
pub mod scene_config;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	pub engine_config_uri: String,
	pub scene_config_uri: String,
}
