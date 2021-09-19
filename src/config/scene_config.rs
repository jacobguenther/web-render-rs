// File: src/config/scene_config.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SceneConfig {
	pub cameras: Vec<CameraConfig>,
	pub current_camera: usize,

	pub shaders: Vec<ShaderConfig>,
	pub programs: Vec<ProgramConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CameraConfig {
	pub id: String,
	pub fov_y: f32,
	pub z_near: f32,
	pub z_far: f32,

	pub eye: [f32; 3],
	pub center: [f32; 3],
	pub up: [f32; 3],
}

#[derive(Clone, Debug, Deserialize)]
pub struct ShaderConfig {
	pub id: String,
	pub kind: String,
	pub path: String,
	pub attributes: Vec<AttributeConfig>,
	pub uniforms: Vec<UniformConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProgramConfig {
	pub id: String,
	pub vertex_id: String,
	pub fragment_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AttributeConfig {
	pub name: String,
	// pub kind: String,
	// pub location: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UniformConfig {
	pub name: String,
	// pub kind: String,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum WebGLType {
	bool,
	int,
	long_int,
	short,
	float,
	double,
	vec2,
	vec3,
	vec4,
	mat2,
	mat3,
	mat4,
}

use crate::model::texture::Sampler;

#[derive(Clone, Debug, Deserialize)]
pub struct ModelConfig {
	pub id: String,
	pub buffers: Vec<BufferConfig>,
	pub meshes: Vec<MeshConfig>,
	pub materials: Vec<MaterialConfig>,
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
pub struct BufferConfig(pub Vec<u8>);

#[derive(Clone, Debug, Deserialize)]
pub struct MeshConfig {
	pub index_view: Option<BufferViewConfig>,
	pub buffer_views: Vec<BufferViewConfig>,
	pub material: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BufferViewConfig {
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
pub struct MaterialConfig {
	pub id: String,
	pub diffuse: Option<u32>,
	pub normal: Option<u32>,
	pub metallic_roughness: Option<u32>,
	pub occlusion: Option<u32>,
}
