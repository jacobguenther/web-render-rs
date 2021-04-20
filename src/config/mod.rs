// File: src/config/mod.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use serde::Deserialize;

use crate::model::texture::Sampler;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	pub canvas_id: String,
	pub width: u32,
	pub height: u32,
	pub camera: CameraIntermediate,
	pub shaders: Vec<ShaderIntermediate>,
	pub programs: Vec<ProgramIntermediate>,
	pub models: Vec<ModelIntermediate>,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct CameraIntermediate {
	pub fovy: f32,
	pub znear: f32,
	pub zfar: f32,

	pub eye: [f32; 3],
	pub center: [f32; 3],
	pub up: [f32; 3],
}
#[derive(Clone, Debug, Deserialize)]
pub struct ShaderIntermediate {
	pub id: String,
	pub shader_type: u32,
	pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProgramIntermediate {
	pub id: String,
	pub vertex: String,
	pub fragment: String,
	pub attributes: Vec<String>,
	pub uniforms: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelIntermediate {
	pub id: String,
	pub buffers: Vec<BufferIntermediate>,
	pub meshes: Vec<MeshIntermediate>,
	pub materials: Vec<MaterialIntermediate>,
	pub texture_wrapper_id: String,
	pub samplers: Vec<Sampler>,
	pub textures: Vec<Texture>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Texture {
	pub source: usize,
	pub sampler: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BufferIntermediate(pub Vec<u8>);

#[derive(Clone, Debug, Deserialize)]
pub struct MeshIntermediate {
	pub index_view: Option<BufferViewIntermediate>,
	pub buffer_views: Vec<BufferViewIntermediate>,
	pub material: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BufferViewIntermediate {
	pub id: String,
	pub buffer: usize,
	pub length: usize,
	pub buffer_offset: Option<i32>,
	pub offset: Option<i32>,
	pub stride: Option<i32>,
	pub component_size: i32,
	pub component_count: i32,
	pub component_type: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MaterialIntermediate {
	pub id: String,
	pub diffuse: u32,
	pub normal: Option<u32>,
	pub metallic_roughness: Option<u32>,
	pub occlusion: Option<u32>,
}
