// File: src/resources.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

pub mod traits;

use std::{collections::HashMap, convert::TryFrom};
use std::{fmt::Debug, rc::Rc};

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use web_sys::{HtmlImageElement, WebGl2RenderingContext};
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::{
	config::{
		BufferViewIntermediate, MaterialIntermediate, ModelIntermediate, ProgramIntermediate,
		ShaderIntermediate,
	},
	model::{
		buffer::Buffer,
		buffer_view::BufferView,
		material::{Material, MetallicRoughnessPart, OcclusionPart},
		mesh::{AttributeBufferViews, Mesh},
		texture::{Sampler, Texture},
		Model,
	},
};

use crate::program::Program;
use crate::shader::Shader;
use crate::warning::*;

use self::traits::{AddResourceT, NewResourceT};

#[derive(Debug)]
pub struct Resources<'c> {
	gl: &'c WebGl2RenderingContext,
	pub strings: HashMap<String, Rc<String>>,
	pub shaders: HashMap<String, Rc<Shader>>,
	pub programs: HashMap<String, Rc<Program>>,

	pub textures: HashMap<String, Rc<Texture>>,
	next_sampler_id: u32,
	pub samplers: HashMap<u32, Rc<Sampler>>,
	next_material_id: u32,
	pub materials: HashMap<u32, Rc<Material>>,

	next_buffer_id: u32,
	pub buffers: HashMap<u32, Rc<Buffer>>,
	next_mesh_id: u32,
	pub meshes: HashMap<u32, Rc<Mesh>>,
	pub models: HashMap<String, Rc<Model>>,
}

impl<'c> AddResourceT for Resources<'c> {
	fn add_string(&mut self, id: &str, string: &str) -> Option<&Rc<String>> {
		self.strings
			.insert(id.to_owned(), Rc::new(string.to_owned()));
		self.strings.get(id)
	}
	fn add_shader(&mut self, id: &str, shader: &Shader) -> Option<&Rc<Shader>> {
		self.shaders
			.insert(id.to_owned(), Rc::new(shader.to_owned()));
		self.shaders.get(id)
	}
	fn add_program(&mut self, id: &str, program: &Program) -> Option<&Rc<Program>> {
		self.programs
			.insert(id.to_owned(), Rc::new(program.to_owned()));
		self.programs.get(id)
	}

	fn add_texture(&mut self, id: &str, texture: &Texture) -> Option<&Rc<Texture>> {
		self.textures
			.insert(id.to_owned(), Rc::new(texture.to_owned()));
		self.textures.get(id)
	}
	fn add_sampler(&mut self, sampler: &Sampler) -> Option<(u32, &Rc<Sampler>)> {
		let id = self.next_sampler_id;
		self.samplers.insert(id, Rc::new(sampler.to_owned()));
		self.next_sampler_id += 1;
		self.samplers.get(&id).map(|s| (id, s))
	}
	fn add_material(&mut self, material: &Material) -> Option<(u32, &Rc<Material>)> {
		let id = self.next_material_id;
		self.materials.insert(id, Rc::new(material.to_owned()));
		self.next_material_id += 1;
		self.materials.get(&id).map(|m| (id, m))
	}

	fn add_buffer(&mut self, buffer: &Buffer) -> Option<(u32, &Rc<Buffer>)> {
		let id = self.next_buffer_id;
		self.buffers.insert(id, Rc::new(buffer.to_owned()));
		self.next_buffer_id += 1;
		self.buffers.get(&id).map(|b| (id, b))
	}
	fn add_mesh(&mut self, mesh: &Mesh) -> Option<(u32, &Rc<Mesh>)> {
		let id = self.next_mesh_id;
		self.meshes.insert(id, Rc::new(mesh.to_owned()));
		self.next_mesh_id += 1;
		self.meshes.get(&id).map(|m| (id, m))
	}
	fn add_model(&mut self, id: &str, model: &Model) -> Option<&Rc<Model>> {
		self.models.insert(id.to_owned(), Rc::new(model.to_owned()));
		self.models.get(id)
	}
}

impl<'c> NewResourceT for Resources<'c> {
	fn new_string(&mut self, id: &str, string: &str) -> Option<&Rc<String>> {
		self.add_string(id, string)
	}
	fn new_shader(
		&mut self,
		id: &str,
		shader_type: u32,
		source: &str,
	) -> Result<&Rc<Shader>, &'static str> {
		if !matches!(
			shader_type,
			WebGl2RenderingContext::VERTEX_SHADER | WebGl2RenderingContext::FRAGMENT_SHADER
		) {
			return Err("Shader type is not a vertex or fragment shader");
		}

		self.add_shader(id, &Shader::new(self.gl, shader_type, source)?)
			.ok_or("Failed to insert shader")
	}
	fn new_program(
		&mut self,
		id: &str,
		vertex: &Shader,
		fragment: &Shader,
		attribute_names: &[String],
		uniform_names: &[String],
	) -> Result<(&Rc<Program>, Vec<ShaderWarning>), String> {
		let (program, warnings) =
			Program::new(self.gl, vertex, fragment, attribute_names, uniform_names)?;

		self.add_program(id, &program)
			.ok_or_else(|| String::from("Failed to insert program"))
			.map(|p| (p, warnings))
	}
	fn new_program_from_shader_ids(
		&mut self,
		id: &str,
		vertex_id: &str,
		fragment_id: &str,
		attribute_names: &[String],
		uniform_names: &[String],
	) -> Result<(&Rc<Program>, Vec<ShaderWarning>), String> {
		let (program, warnings) = {
			let vertex = self
				.shaders
				.get(vertex_id)
				.ok_or_else(|| String::from("Resources is missing vertex shader"))?;
			let fragment = self
				.shaders
				.get(fragment_id)
				.ok_or_else(|| String::from("Resources is missing fragment shader"))?;
			Program::new(self.gl, vertex, fragment, attribute_names, uniform_names)?
		};
		self.add_program(id, &program)
			.ok_or_else(|| String::from("Failed to insert program"))
			.map(|p| (p, warnings))
	}

	fn new_texture(
		&mut self,
		id: &str,
		image_element: &HtmlImageElement,
		texture_unit: u32,
		sampler: &Rc<Sampler>,
	) -> Result<&Rc<Texture>, &'static str> {
		let texture = Texture::new(self.gl, image_element, texture_unit, sampler)?;
		self.add_texture(id, &texture)
			.ok_or("Failed to add new texture")
	}
	fn new_material(
		&mut self,
		material: &MaterialIntermediate,
		textures: &[Rc<Texture>],
	) -> Result<(u32, &Rc<Material>), &'static str> {
		let diffuse_tex = textures.get(material.diffuse as usize).cloned();

		let normal_tex = material
			.normal
			.map(|i| textures.get(i as usize).cloned())
			.flatten();

		let metallic_roughness_part = material
			.metallic_roughness
			.map(|i| {
				textures
					.get(i as usize)
					.map(|t| MetallicRoughnessPart::Texture(Rc::clone(t)))
			})
			.flatten()
			.unwrap_or_else(MetallicRoughnessPart::default);

		let occlusion_part = material
			.occlusion
			.map(|i| {
				textures
					.get(i as usize)
					.map(|t| OcclusionPart::Texture(Rc::clone(t)))
			})
			.flatten()
			.unwrap_or_else(OcclusionPart::default);

		let mat = Material {
			diffuse_tex,
			normal_tex,
			metallic_roughness_part,
			occlusion_part,
		};

		self.add_material(&mat).ok_or("Failed to add material")
	}

	fn new_buffer(
		&mut self,
		buffer_type: u32,
		data: &[u8],
	) -> Result<(u32, &Rc<Buffer>), &'static str> {
		let buffer = Buffer::new(self.gl, buffer_type, data)?;
		self.add_buffer(&buffer).ok_or("Failed to add buffer.")
	}
	fn new_mesh(
		&mut self,
		material: &Rc<Material>,
		buffers: &[Rc<Buffer>],
		index_view: &Option<BufferViewIntermediate>,
		buffer_views: &[BufferViewIntermediate],
	) -> Result<(u32, &Rc<Mesh>), &'static str> {
		let index_view = index_view.clone().map(|ref i| BufferView::new(i));
		let attribute_buffer_views = AttributeBufferViews::try_from(buffer_views)?;
		let mesh = Mesh::new(
			self.gl,
			material,
			buffers,
			&index_view,
			&attribute_buffer_views,
		)?;
		self.add_mesh(&mesh).ok_or("Failed to add mesh.")
	}
	fn new_model(&mut self, id: &str, meshes: &[Rc<Mesh>]) -> Result<&Rc<Model>, &'static str> {
		let model = Model::new(meshes);
		self.add_model(id, &model).ok_or("Failed to add model")
	}
}

impl<'c> Resources<'c> {
	pub fn new(gl: &'c WebGl2RenderingContext) -> Self {
		Self {
			gl,
			strings: HashMap::new(),
			shaders: HashMap::new(),
			programs: HashMap::new(),
			textures: HashMap::new(),
			next_sampler_id: 0,
			samplers: HashMap::new(),
			next_buffer_id: 0,
			buffers: HashMap::new(),
			next_material_id: 0,
			materials: HashMap::new(),
			next_mesh_id: 0,
			meshes: HashMap::new(),
			models: HashMap::new(),
		}
	}
	pub async fn load_texts(&mut self, sources: &[&str]) -> Result<Vec<Rc<String>>, JsValue> {
		let mut futures = Vec::with_capacity(sources.len());
		let mut texts = Vec::with_capacity(sources.len());
		for &source in sources.iter() {
			if !self.strings.contains_key(source) {
				let opts = {
					let mut temp = RequestInit::new();
					temp.method("GET");
					temp.mode(RequestMode::SameOrigin);
					temp
				};
				let uri = format!("assets/{}", source);
				let request = Request::new_with_str_and_init(&uri, &opts)?;
				request.headers().set("Accept", "application/json")?;
				let window = web_sys::window().unwrap();
				let response_value = JsFuture::from(window.fetch_with_request(&request));
				futures.push((source, response_value));
			} else {
				texts.push(self.strings.get(source).unwrap().clone());
			}
		}
		for (source, future) in futures.into_iter() {
			let response_value = future.await?;
			assert!(response_value.is_instance_of::<Response>());
			let response: Response = response_value.dyn_into().unwrap();
			let text = JsFuture::from(response.text()?).await?.as_string().unwrap();
			texts.push(Rc::clone(self.new_string(source, &text).unwrap()));
		}
		Ok(texts)
	}
	pub async fn load_shaders(
		&mut self,
		data: &[ShaderIntermediate],
	) -> Result<Vec<Rc<Shader>>, &'static str> {
		let sources = data
			.iter()
			.map(|info| info.uri.as_str())
			.collect::<Vec<_>>();
		self.load_texts(&sources)
			.await
			.map_err(|_| "Error fetching shader source")?;

		let mut shaders = Vec::with_capacity(data.len());
		for info in data.iter() {
			let shader_source = self.strings.get(&info.uri).unwrap().clone();
			shaders.push(Rc::clone(self.new_shader(
				&info.id,
				info.shader_type,
				&shader_source,
			)?));
		}
		Ok(shaders)
	}
	pub fn load_programs(
		&mut self,
		data: &[ProgramIntermediate],
	) -> Result<(Vec<Rc<Program>>, Vec<ShaderWarning>), String> {
		let mut programs = Vec::with_capacity(data.len());
		let mut warnings = Vec::new();
		for info in data.iter() {
			let (program, mut program_warnings) = self.new_program_from_shader_ids(
				&info.id,
				&info.vertex,
				&info.fragment,
				&info.attributes,
				&info.uniforms,
			)?;
			warnings.append(&mut program_warnings);
			programs.push(Rc::clone(program));
		}
		Ok((programs, warnings))
	}
	pub fn load_models(
		&mut self,
		models: &[ModelIntermediate],
	) -> Result<Vec<Rc<Model>>, &'static str> {
		let mut ret = Vec::with_capacity(models.len());
		for model_data in models.iter() {
			let mut samplers = Vec::with_capacity(model_data.samplers.len());
			for sampler in model_data.samplers.iter() {
				let (_sampler_id, sampler) = self.add_sampler(sampler).unwrap();
				samplers.push(Rc::clone(sampler));
			}
			let collection_id = format!("{}-images", model_data.id);
			let collection = web_sys::window()
				.unwrap()
				.document()
				.unwrap()
				.get_element_by_id(&collection_id)
				.unwrap()
				.children();
			let texture_count = collection.length();
			let mut image_elements = Vec::new();
			for i in 0..texture_count {
				let image_element = collection
					.item(i)
					.unwrap()
					.dyn_into::<HtmlImageElement>()
					.unwrap();
				image_elements.push(image_element);
			}

			let mut textures = Vec::with_capacity(texture_count as usize);
			for texture in model_data.textures.iter() {
				let image_element = &image_elements[texture.source];
				let texture_id = image_element.id();
				let sampler = &samplers[texture.sampler];
				let texture = self.new_texture(&texture_id, image_element, 0, sampler)?;
				textures.push(Rc::clone(texture));
			}

			let mut materials = Vec::with_capacity(model_data.materials.len());
			for material in model_data.materials.iter() {
				let (_material_id, material) = self.new_material(material, &textures)?;
				materials.push(Rc::clone(material));
			}

			let mut buffers = Vec::with_capacity(model_data.buffers.len() + 1);
			for buffer_data in model_data.buffers.iter() {
				let (_buffer_id, buffer) =
					self.new_buffer(WebGl2RenderingContext::ARRAY_BUFFER, &buffer_data.0)?;
				buffers.push(Rc::clone(buffer));
			}

			let mut meshes = Vec::with_capacity(model_data.meshes.len());
			for mesh in model_data.meshes.iter() {
				// Clean this up
				let index_view = match mesh.index_view {
					Some(ref i) => {
						let offset = i.buffer_offset.unwrap_or(0) as usize;
						let index_buffer_size = offset + i.length;
						let slice = &model_data.buffers[i.buffer].0[offset..index_buffer_size];
						let (_index_buffer_id, index_buffer) =
							self.new_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, slice)?;
						buffers.push(Rc::clone(index_buffer));
						let mut view = i.clone();
						view.buffer = buffers.len() - 1;
						Some(view)
					}
					None => None,
				};
				let material = if let Some(material_index) = mesh.material {
					materials[material_index as usize].clone()
				} else {
					Rc::new(Material::default())
				};
				let (_mesh_id, mesh) =
					self.new_mesh(&material, &buffers, &index_view, &mesh.buffer_views)?;
				meshes.push(Rc::clone(mesh));
			}
			let model = self.new_model(&model_data.id, &meshes)?;
			ret.push(Rc::clone(model));
		}
		Ok(ret)
	}
}

// pub async fn fetch(path: &str) -> Result<String, JsValue> {
// 	let mut opts = RequestInit::new();
// 	opts.method("GET");
// 	opts.mode(RequestMode::SameOrigin);

// 	let uri = format!("assets/{}", path);
// 	let request = Request::new_with_str_and_init(&uri, &opts)?;

// 	request.headers().set("Accept", "application/json")?;

// 	let window = web_sys::window().unwrap();
// 	let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;

// 	assert!(response_value.is_instance_of::<Response>());

// 	let response: Response = response_value.dyn_into().unwrap();

// 	let text = JsFuture::from(response.text()?).await?.as_string().unwrap();

// 	Ok(text)
// }
