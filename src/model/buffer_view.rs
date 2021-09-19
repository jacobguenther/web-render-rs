// File: src/model/buffer_view.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use crate::config::scene_config::BufferViewConfig;

#[derive(Copy, Clone, Debug)]
pub struct BufferView {
	pub buffer: usize,
	pub length: usize,
	pub buffer_offset: i32,
	pub offset: i32,
	pub combined_offset: i32,
	pub stride: i32,
	pub component_size: i32,
	pub component_count: i32,
	pub component_type: u32,
}
impl BufferView {
	pub fn new(intermediate: &BufferViewConfig) -> Self {
		let buffer_offset = intermediate.buffer_offset.unwrap_or(0);
		let offset = intermediate.offset.unwrap_or(0);
		Self {
			buffer: intermediate.buffer,
			length: intermediate.length,
			buffer_offset,
			offset,
			combined_offset: buffer_offset + offset,
			stride: intermediate.stride.unwrap_or(0),
			component_size: intermediate.component_size,
			component_count: intermediate.component_count,
			component_type: intermediate.component_type,
		}
	}
}
