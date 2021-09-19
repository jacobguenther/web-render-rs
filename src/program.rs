// File: src/program.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use std::collections::HashMap;

use web_sys::{
	WebGl2RenderingContext,
	WebGlProgram,
	WebGlShader,
	WebGlUniformLocation,
};

use crate::config::scene_config::{
	AttributeConfig,
	UniformConfig,
};
use crate::shader::Shader;
use crate::warning::*;

pub mod attribute_locations {
	pub const POSITION_LOCATION: u32 = 0;
	pub const NORMAL_LOCATION: u32 = 1;
	pub const TANGENT_LOCATION: u32 = 2;
	pub const BITANGENT_LOCATION: u32 = 3;
	pub const COLOR_LOCATION: u32 = 4;
	pub const TEXCORD_0_LOCATION: u32 = 5;
	pub const TEXCORD_1_LOCATION: u32 = 6;
	pub const TEXCORD_2_LOCATION: u32 = 7;
	pub const TEXCORD_3_LOCATION: u32 = 8;
}

#[derive(Clone, Debug)]
pub struct Program {
	pub program: WebGlProgram,
	pub attribute_locations: HashMap<String, u32>,
	pub uniform_locations: HashMap<String, WebGlUniformLocation>,
}
impl Program {
	pub fn new(
		gl: &WebGl2RenderingContext,
		vert: &Shader,
		frag: &Shader,
	) -> Result<(Self, Vec<ShaderWarning>), String> {
		let program = Self::link_program(gl, &vert.handle, &frag.handle)?;

		let (attribute_locations, attribute_warnings) =
			Self::attribute_locations(gl, &program, &vert.attributes)?;

		let uniforms = {
			let mut temp = vert.uniforms.clone();
			temp.extend(frag.uniforms.iter().cloned());
			temp
		};
		let (uniform_locations, uniform_warnings) =
			Self::uniform_locations(gl, &program, &uniforms)?;

		let warnings = attribute_warnings
			.iter()
			.chain(uniform_warnings.iter())
			.cloned()
			.collect();

		Ok((
			Self {
				program,
				attribute_locations,
				uniform_locations,
			},
			warnings,
		))
	}

	fn link_program(
		gl: &WebGl2RenderingContext,
		vert_shader: &WebGlShader,
		frag_shader: &WebGlShader,
	) -> Result<WebGlProgram, String> {
		let program = gl
			.create_program()
			.ok_or("Unable to create shader object")?;

		gl.attach_shader(&program, vert_shader);
		gl.attach_shader(&program, frag_shader);
		gl.link_program(&program);

		if gl
			.get_program_parameter(
				&program,
				WebGl2RenderingContext::LINK_STATUS,
			)
			.as_bool()
			.unwrap_or(false)
		{
			gl.validate_program(&program);
			if gl
				.get_program_parameter(
					&program,
					WebGl2RenderingContext::VALIDATE_STATUS,
				)
				.as_bool()
				.unwrap_or(false)
			{
				Ok(program)
			} else {
				Err(Self::get_logs(gl, &program, &vert_shader, &frag_shader))
			}
		} else {
			Err(Self::get_logs(gl, &program, &vert_shader, &frag_shader))
		}
	}
	fn get_logs(
		gl: &WebGl2RenderingContext,
		program: &WebGlProgram,
		vert_shader: &WebGlShader,
		frag_shader: &WebGlShader,
	) -> String {
		let program_log = gl
			.get_program_info_log(&program)
			.unwrap_or_else(|| String::from("Unable to get program logs"));
		let vert_log =
			gl.get_shader_info_log(vert_shader).unwrap_or_else(|| {
				String::from("Unable to get vertex shader logs")
			});
		let frag_log =
			gl.get_shader_info_log(frag_shader).unwrap_or_else(|| {
				String::from("Unable to get frabment shader logs")
			});
		let mut logs = String::from("Program Log:\n");
		logs.push_str(&program_log);
		logs.push_str("\nVertex Log:\n");
		logs.push_str(&vert_log);
		logs.push_str("\nFragment Log:\n");
		logs.push_str(&frag_log);
		logs.push('\n');
		logs
	}
	fn attribute_locations(
		gl: &WebGl2RenderingContext,
		program: &WebGlProgram,
		attributes: &[AttributeConfig],
	) -> Result<(HashMap<String, u32>, Vec<ShaderWarning>), &'static str> {
		let mut locations = HashMap::new();
		let mut warnings = Vec::new();
		for attribute in attributes.iter() {
			let location =
				gl.get_attrib_location(program, attribute.name.as_str());
			if location < 0 {
				warnings.push(ShaderWarning::AttributeNotFound(
					attribute.name.clone(),
				));
				continue;
			}
			locations.insert(attribute.name.clone(), location as u32);
		}
		Ok((locations, warnings))
	}
	fn uniform_locations(
		gl: &WebGl2RenderingContext,
		program: &WebGlProgram,
		uniforms: &[UniformConfig],
	) -> Result<
		(HashMap<String, WebGlUniformLocation>, Vec<ShaderWarning>),
		&'static str,
	> {
		let mut locations = HashMap::new();
		let mut warnings = Vec::new();
		locations.reserve(uniforms.len());
		for uniform in uniforms.iter() {
			let location =
				match gl.get_uniform_location(program, uniform.name.as_str()) {
					Some(location) => location,
					None => {
						warnings.push(ShaderWarning::UniformNotFound(
							uniform.name.clone(),
						));
						continue;
					}
				};
			locations.insert(uniform.name.clone(), location);
		}
		Ok((locations, warnings))
	}
}
