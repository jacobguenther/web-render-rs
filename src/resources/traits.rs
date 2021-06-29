// File: src/resources.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use web_sys::{
	Document,
	HtmlImageElement,
};

use crate::{
	config::{
		BufferViewIntermediate,
		MaterialIntermediate,
	},
	model::{
		buffer::Buffer,
		material::Material,
		mesh::Mesh,
		texture::{
			Sampler,
			Texture,
		},
		Model,
	},
	program::Program,
};
use crate::{
	shader::Shader,
	warning::ShaderWarning,
};

use std::rc::Rc;

pub trait AddResourceT {
	fn add_string(&mut self, id: &str, string: &str) -> Option<&Rc<String>>;
	fn add_shader(&mut self, id: &str, shader: &Shader) -> Option<&Rc<Shader>>;
	fn add_program(
		&mut self,
		id: &str,
		program: &Program,
	) -> Option<&Rc<Program>>;

	fn add_texture(
		&mut self,
		id: &str,
		texture: &Texture,
	) -> Option<&Rc<Texture>>;
	fn add_sampler(&mut self, sampler: &Sampler)
		-> Option<(u32, &Rc<Sampler>)>;
	fn add_material(
		&mut self,
		material: &Material,
	) -> Option<(u32, &Rc<Material>)>;

	fn add_buffer(&mut self, buffer: &Buffer) -> Option<(u32, &Rc<Buffer>)>;
	fn add_mesh(&mut self, mesh: &Mesh) -> Option<(u32, &Rc<Mesh>)>;
	fn add_model(&mut self, id: &str, model: &Model) -> Option<&Rc<Model>>;
}
pub trait NewResourceT {
	fn new_string(&mut self, id: &str, string: &str) -> Option<&Rc<String>>;
	fn new_shader(
		&mut self,
		id: &str,
		shader_type: u32,
		shader_source: &str,
	) -> Result<&Rc<Shader>, &'static str>;
	fn new_program(
		&mut self,
		id: &str,
		vertex: &Shader,
		fragment: &Shader,
		attribute_names: &[String],
		uniform_names: &[String],
	) -> Result<(&Rc<Program>, Vec<ShaderWarning>), String>;
	fn new_program_from_shader_ids(
		&mut self,
		id: &str,
		vertex_id: &str,
		fragment_id: &str,
		attribute_names: &[String],
		uniform_names: &[String],
	) -> Result<(&Rc<Program>, Vec<ShaderWarning>), String>;

	fn new_texture(
		&mut self,
		id: &str,
		image_element: &HtmlImageElement,
		texture_unit: u32,
		sampler: &Rc<Sampler>,
	) -> Result<&Rc<Texture>, &'static str>;
	fn new_material(
		&mut self,
		material: &MaterialIntermediate,
		textures: &[Rc<Texture>],
	) -> Result<(u32, &Rc<Material>), &'static str>;

	fn new_buffer(
		&mut self,
		buffer_type: u32,
		data: &[u8],
	) -> Result<(u32, &Rc<Buffer>), &'static str>;
	fn new_mesh(
		&mut self,
		material: &Rc<Material>,
		buffers: &[Rc<Buffer>],
		index_view: &Option<BufferViewIntermediate>,
		buffer_views: &[BufferViewIntermediate],
		mode: u32,
	) -> Result<(u32, &Rc<Mesh>), &'static str>;
	fn new_model(
		&mut self,
		id: &str,
		meshes: &[Rc<Mesh>],
	) -> Result<&Rc<Model>, &'static str>;
}
pub trait LoadResourceT {
	// fn fetch_file();
	// fn batch_fetch_files();

	fn fetch_image(
		document: &Document,
		id: &str,
		uri: &str,
		parent_id: &str,
	) -> Result<HtmlImageElement, &'static str>;

	// fn load_config();
	// fn load_model();
	// fn load_mesh();
}
pub trait GetResourceT {
	fn get_text(id: &str) -> Option<String>;
	fn get_shader(id: &str) -> Option<Shader>;
	fn get_program(id: &str) -> Option<Program>;
}
pub trait RemoveResourceT {
	fn remove_text(id: &str);
	fn remove_shader(id: &str);
	fn remove_program(id: &str);
}
