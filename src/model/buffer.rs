// File: src/model/buffer.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[derive(Clone, Debug)]
pub struct Buffer {
	pub handle: WebGlBuffer,
	pub buffer_type: u32,
}

impl Buffer {
	pub fn new(
		gl: &WebGl2RenderingContext,
		buffer_type: u32,
		data: &[u8],
	) -> Result<Self, &'static str> {
		let handle = gl.create_buffer().ok_or("failed to create buffer")?;
		gl.bind_buffer(buffer_type, Some(&handle));
		Self::bind_data(gl, buffer_type, data);

		Ok(Self {
			handle,
			buffer_type,
		})
	}
	pub fn bind(&self, gl: &WebGl2RenderingContext) {
		gl.bind_buffer(self.buffer_type, Some(&self.handle));
	}
	fn bind_data(gl: &WebGl2RenderingContext, buffer_type: u32, data: &[u8]) {
		unsafe {
			let array = js_sys::Uint8Array::view(&data);
			gl.buffer_data_with_array_buffer_view(
				buffer_type,
				&array,
				WebGl2RenderingContext::STATIC_DRAW,
			);
		}
	}
}
