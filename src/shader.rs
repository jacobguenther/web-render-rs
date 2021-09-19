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

use crate::config::scene_config::{
	AttributeConfig,
	UniformConfig,
};

#[derive(Clone, Debug)]
pub struct Shader {
	pub shader_type: u32,
	pub handle: WebGlShader,
	pub attributes: Vec<AttributeConfig>,
	pub uniforms: Vec<UniformConfig>,
}
impl Shader {
	pub fn new(
		gl: &WebGl2RenderingContext,
		shader_type: u32,
		source: &str,
		attributes: &[AttributeConfig],
		uniforms: &[UniformConfig],
	) -> Result<Self, &'static str> {
		Ok(Self {
			shader_type,
			handle: Self::compile_shader(gl, shader_type, source)?,
			attributes: attributes.to_owned(),
			uniforms: uniforms.to_owned(),
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
