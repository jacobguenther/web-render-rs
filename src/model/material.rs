// File: src/model/mesh.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use std::collections::HashMap;
use std::rc::Rc;

use web_sys::{
	WebGl2RenderingContext,
	WebGlUniformLocation,
};

use super::texture::Texture;

#[derive(Clone, Debug)]
pub struct Material {
	pub diffuse_tex: Option<Rc<Texture>>,
	pub normal_tex: Option<Rc<Texture>>,
	pub metallic_roughness_part: MetallicRoughnessPart,
	pub occlusion_part: OcclusionPart,
}
impl Default for Material {
	fn default() -> Self {
		Self {
			diffuse_tex: None,
			normal_tex: None,
			metallic_roughness_part: MetallicRoughnessPart::default(),
			occlusion_part: OcclusionPart::default(),
		}
	}
}
impl Material {
	pub fn bind_to_uniforms(
		&self,
		gl: &WebGl2RenderingContext,
		uniform_locations: &HashMap<String, WebGlUniformLocation>,
	) {
		let mut texture_unit = 0;

		let use_diffuse_tex = if let Some(ref diffuse_tex) = self.diffuse_tex {
			let diffuse_tex_loc = uniform_locations.get("DIFFUSE_TEX");
			gl.uniform1i(diffuse_tex_loc, texture_unit);
			diffuse_tex.bind(gl, texture_unit as u32);
			texture_unit += 1;
			true
		} else {
			false
		};
		let use_diffuse_tex_loc = uniform_locations.get("USE_DIFFUSE_TEX");
		gl.uniform1i(use_diffuse_tex_loc, use_diffuse_tex as i32);

		let use_normal_tex = if let Some(ref normal_tex) = self.normal_tex {
			let normal_tex_loc = uniform_locations.get("NORMAL_TEX");
			gl.uniform1i(normal_tex_loc, texture_unit);
			normal_tex.bind(gl, texture_unit as u32);
			texture_unit += 1;
			true
		} else {
			false
		};
		let use_normal_tex_loc = uniform_locations.get("USE_NORMAL_TEX");
		gl.uniform1i(use_normal_tex_loc, use_normal_tex as i32);

		let use_matallic_roughness_tex = match self.metallic_roughness_part {
			MetallicRoughnessPart::Texture(ref t) => {
				let metallic_roughness_loc =
					uniform_locations.get("METALLIC_ROUGHNESS_TEX");
				gl.uniform1i(metallic_roughness_loc, texture_unit);
				t.bind(gl, texture_unit as u32);
				texture_unit += 1;
				true
			}
			MetallicRoughnessPart::Values(m, r) => {
				let metallic_loc = uniform_locations.get("METALLIC");
				let roughness_loc = uniform_locations.get("ROUGHNESS");
				gl.uniform1f(metallic_loc, m);
				gl.uniform1f(roughness_loc, r);
				false
			}
		};
		let use_metallic_roughness_tex_loc =
			uniform_locations.get("USE_METALLIC_ROUGHNESS_TEX");
		gl.uniform1i(
			use_metallic_roughness_tex_loc,
			use_matallic_roughness_tex as i32,
		);

		let use_occlusion_tex = match self.occlusion_part {
			OcclusionPart::Texture(ref t) => {
				let occlusion_tex_loc = uniform_locations.get("OCCLUSION_TEX");
				gl.uniform1i(occlusion_tex_loc, texture_unit);
				t.bind(gl, texture_unit as u32);
				// texture_unit += 1;
				true
			}
			OcclusionPart::Value(ao) => {
				let occlusion_loc = uniform_locations.get("OCCLUSION");
				gl.uniform1f(occlusion_loc, ao);
				false
			}
		};
		let use_occlusion_tex_loc = uniform_locations.get("USE_OCCLUSION_TEX");
		gl.uniform1i(use_occlusion_tex_loc, use_occlusion_tex as i32);
	}
}

#[derive(Clone, Debug)]
pub enum MetallicRoughnessPart {
	Texture(Rc<Texture>),
	Values(f32, f32),
}
impl Default for MetallicRoughnessPart {
	fn default() -> Self {
		MetallicRoughnessPart::Values(0.0, 0.0)
	}
}

#[derive(Clone, Debug)]
pub enum OcclusionPart {
	Texture(Rc<Texture>),
	Value(f32),
}
impl Default for OcclusionPart {
	fn default() -> Self {
		OcclusionPart::Value(0.0)
	}
}
