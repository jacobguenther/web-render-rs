// File: src/shaders.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use web_sys::{
	WebGl2RenderingContext,
	WebGlShader,
};

#[derive(Clone, Debug)]
pub struct Shader {
	pub shader_type: u32,
	pub handle: WebGlShader,
}
impl Shader {
	pub fn new(
		gl: &WebGl2RenderingContext,
		shader_type: u32,
		source: &str,
	) -> Result<Self, &'static str> {
		Ok(Self {
			shader_type,
			handle: Self::compile_shader(gl, shader_type, source)?,
		})
	}
	fn compile_shader(
		gl: &WebGl2RenderingContext,
		shader_type: u32,
		source: &str,
	) -> Result<WebGlShader, &'static str> {
		let shader = gl
			.create_shader(shader_type)
			.ok_or("Unable to create shader object")?;
		gl.shader_source(&shader, source);
		gl.compile_shader(&shader);
		Ok(shader)
	}
}
