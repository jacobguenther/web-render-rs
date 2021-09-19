// File: src/model/terrain.rs
// Author: Jacob Guenther
// Date created: April 2021
// License: AGPLv3
//
// Description:

pub mod cube;
pub mod face;
// pub mod icosphere;
pub mod superellipse;
pub mod supershape_2d;
pub mod terrain;
pub mod uv_sphere;
pub mod vertex;

use std::{
	cell::RefCell,
	mem,
	rc::Rc,
};

use cgmath::{
	InnerSpace,
	Vector3,
	Zero,
};

use web_sys::WebGl2RenderingContext;

use crate::model::{
	buffer::Buffer,
	buffer_view::BufferView,
	material::Material,
	mesh::{
		AttributeBufferViews,
		Mesh,
	},
};

use face::Face;
use vertex::Vertex;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MeshMode {
	Points,
	TriangleFan,
	Triangles,
	IndexedTriangles,
}

pub trait MeshGeneratorT {
	fn generate_mesh(
		&mut self,
		gl: &WebGl2RenderingContext,
		mode: MeshMode,
	) -> Mesh;
	fn generate_rc_mesh(
		&mut self,
		gl: &WebGl2RenderingContext,
		mode: MeshMode,
	) -> Rc<Mesh> {
		Rc::new(self.generate_mesh(gl, mode))
	}
	fn generate_refcell_mesh(
		&mut self,
		gl: &WebGl2RenderingContext,
		mode: MeshMode,
	) -> RefCell<Mesh> {
		RefCell::new(self.generate_mesh(gl, mode))
	}
	fn generate_rc_refcell_mesh(
		&mut self,
		gl: &WebGl2RenderingContext,
		mode: MeshMode,
	) -> Rc<RefCell<Mesh>> {
		Rc::new(self.generate_refcell_mesh(gl, mode))
	}
}
#[derive(Debug)]
pub struct MeshGenerator {
	vertices: Vec<Vertex>,
	indices: Option<Vec<u32>>,
	vertex_face_adjacency_list: Option<Vec<Vec<usize>>>,
	faces_hint: usize,
}
impl Default for MeshGenerator {
	fn default() -> Self {
		let default_capacity = 16;
		Self {
			vertices: Vec::with_capacity(default_capacity),
			indices: None,
			vertex_face_adjacency_list: None,
			faces_hint: default_capacity,
		}
	}
}
impl MeshGeneratorT for MeshGenerator {
	fn generate_mesh(
		&mut self,
		gl: &WebGl2RenderingContext,
		mode: MeshMode,
	) -> Mesh {
		let (buffers, index_buffer_view, attribute_buffer_views, mode) =
			match mode {
				MeshMode::Points => {
					let (attribute_buffer, attribute_buffer_views) =
						self.create_attribute_buffer(gl);

					if false {
						let (index_buffer, index_buffer_view) =
							self.create_index_buffer(gl);

						(
							vec![attribute_buffer, index_buffer],
							Some(index_buffer_view),
							attribute_buffer_views,
							WebGl2RenderingContext::POINTS,
						)
					} else {
						(
							vec![attribute_buffer],
							None,
							attribute_buffer_views,
							WebGl2RenderingContext::POINTS,
						)
					}
				}
				MeshMode::TriangleFan => {
					let (attribute_buffer, attribute_buffer_views) =
						self.create_attribute_buffer(gl);
					(
						vec![attribute_buffer],
						None,
						attribute_buffer_views,
						WebGl2RenderingContext::TRIANGLE_FAN,
					)
				}
				MeshMode::Triangles => {
					let faces = self.create_faces();
					if self.vertex_face_adjacency_list.is_none() {
						self.vertex_face_adjacency_list = Some(
							self.create_vertex_face_adjacency_list(&faces),
						);
					}
					self.create_normals_and_tangents(&faces);
					let (attribute_buffer, attribute_buffer_views) =
						self.create_attribute_buffer(gl);

					(
						vec![attribute_buffer],
						None,
						attribute_buffer_views,
						WebGl2RenderingContext::TRIANGLES,
					)
				}
				MeshMode::IndexedTriangles => {
					if self.vertices[0].normal.is_none() {
						let faces = self.create_faces();
						if self.vertex_face_adjacency_list.is_none() {
							self.vertex_face_adjacency_list = Some(
								self.create_vertex_face_adjacency_list(&faces),
							);
						}
						self.create_normals_and_tangents(&faces);
					}

					let (attribute_buffer, attribute_buffer_views) =
						self.create_attribute_buffer(gl);

					let (index_buffer, index_buffer_view) =
						self.create_index_buffer(gl);

					(
						vec![attribute_buffer, index_buffer],
						Some(index_buffer_view),
						attribute_buffer_views,
						WebGl2RenderingContext::TRIANGLES,
					)
				}
			};
		let material = Rc::new(Material::default());
		Mesh::new(
			gl,
			&material,
			&buffers,
			&index_buffer_view,
			&attribute_buffer_views,
			mode,
		)
		.unwrap()
	}
}
impl MeshGenerator {
	pub fn new(
		vertices_hint: Option<usize>,
		indices_hint: Option<usize>,
		faces_hint: Option<usize>,
	) -> Self {
		let default_capacity = 16;
		let indices = indices_hint.map(Vec::with_capacity);
		let vertices =
			vertices_hint.map_or(Vec::with_capacity(16), Vec::with_capacity);
		Self {
			vertices,
			indices,
			vertex_face_adjacency_list: None,
			faces_hint: faces_hint.unwrap_or(default_capacity),
		}
	}
	pub fn push_vertex(&mut self, vertex: &Vertex) {
		self.vertices.push(*vertex);
	}
	pub fn push_index(&mut self, index: u32) {
		if let Some(ref mut indices) = self.indices {
			indices.push(index);
		} else {
			self.indices = Some(vec![index]);
		}
	}
	pub fn push_index_slice(&mut self, new_indices: &[u32]) {
		if let Some(ref mut indices) = self.indices {
			indices.extend_from_slice(new_indices);
		} else {
			self.indices = Some(new_indices.to_vec());
		}
	}
	pub fn push_adjacent_faces(&mut self, adjacent_faces: &[usize]) {
		if let Some(ref mut vertex_face_adjacency_list) =
			self.vertex_face_adjacency_list
		{
			vertex_face_adjacency_list.push(adjacent_faces.to_owned());
		} else {
			let mut vertex_face_adjacency_list =
				Vec::with_capacity(self.vertices.len());
			vertex_face_adjacency_list.push(adjacent_faces.to_owned());
			self.vertex_face_adjacency_list = Some(vertex_face_adjacency_list);
		}
	}
	pub fn mut_ref_vertices(&mut self) -> &mut Vec<Vertex> {
		&mut self.vertices
	}
	pub fn mut_ref_indices(&mut self) -> &mut Vec<u32> {
		if let Some(ref mut indices) = self.indices {
			indices
		} else {
			self.indices = Some(Vec::new());
			self.indices.as_mut().unwrap()
		}
	}
	pub fn mut_ref_vertex_face_adjacency_list(
		&mut self,
	) -> &mut Option<Vec<Vec<usize>>> {
		&mut self.vertex_face_adjacency_list
	}

	fn create_faces(&self) -> Vec<Face> {
		let mut faces = Vec::with_capacity(self.faces_hint);
		if let Some(ref indices) = self.indices {
			for i in (0..indices.len()).step_by(3) {
				let vertex_0_index = indices[i] as usize;
				let vertex_1_index = indices[i + 1] as usize;
				let vertex_2_index = indices[i + 2] as usize;
				let face = Face::new(
					&self.vertices,
					vertex_0_index,
					vertex_1_index,
					vertex_2_index,
				);
				faces.push(face);
			}
		} else {
			for i in (0..self.vertices.len()).step_by(3) {
				let vertex_0_index = i;
				let vertex_1_index = i + 1;
				let vertex_2_index = i + 2;
				let face = Face::new(
					&self.vertices,
					vertex_0_index,
					vertex_1_index,
					vertex_2_index,
				);
				faces.push(face);
			}
		}
		faces
	}
	// vertex_face_adjacency_list[vertex_index][face_index]
	fn create_vertex_face_adjacency_list(
		&self,
		faces: &[Face],
	) -> Vec<Vec<usize>> {
		let mut adjacency_list = Vec::with_capacity(self.vertices.len());
		for vertex_index in 0..self.vertices.len() {
			let mut adjacent_faces = Vec::new();
			for (face_index, face) in faces.iter().enumerate() {
				if vertex_index == face.vertex_0_index
					|| vertex_index == face.vertex_1_index
					|| vertex_index == face.vertex_2_index
				{
					adjacent_faces.push(face_index);
				}
			}
			adjacency_list.push(adjacent_faces);
		}
		adjacency_list
	}
	fn create_normals_and_tangents(&mut self, faces: &[Face]) {
		let vertex_face_adjacency_list =
			self.vertex_face_adjacency_list.as_ref().unwrap();
		if self.vertices.len() != vertex_face_adjacency_list.len() {
			panic!(
				"vertices and vertex_face_adjacency_list must have same length"
			);
		}
		self.vertices
			.iter_mut()
			.enumerate()
			.for_each(|(i, vertex)| {
				let adjacent_faces = &vertex_face_adjacency_list[i];
				let normal = adjacent_faces
					.iter()
					.fold(Vector3::zero(), |acc, f| acc + faces[*f].normal);
				vertex.normal = Some(normal.normalize());

				let tangent = adjacent_faces
					.iter()
					.fold(Vector3::zero(), |acc, f| acc + faces[*f].tangent);
				vertex.tangent = Some(tangent.normalize());
			});
	}
	fn create_attribute_buffer(
		&self,
		gl: &WebGl2RenderingContext,
	) -> (Rc<Buffer>, AttributeBufferViews) {
		let vertex_count = self.vertices.len();
		let mut data_f32 = Vec::with_capacity(
			vertex_count * self.vertices[0].stride() as usize,
		);
		self.vertices
			.iter()
			.for_each(|v| data_f32.extend_from_slice(&v.as_slice()));

		let ratio = mem::size_of::<f32>() / mem::size_of::<u8>();
		let length = data_f32.len() * ratio;
		let ptr = data_f32.as_ptr() as *mut u8;
		let vertex_data = unsafe { std::slice::from_raw_parts(ptr, length) };

		let attribute_buffer = Rc::new(
			Buffer::new(gl, WebGl2RenderingContext::ARRAY_BUFFER, vertex_data)
				.unwrap(),
		);

		let position_view = self.vertices[0].position_view(vertex_count);
		let normal_view = self.vertices[0].normal_view(vertex_count);
		let tangent_view = self.vertices[0].tangent_view(vertex_count);
		let texcoord_0_view = self.vertices[0].texcoord_0_view(vertex_count);

		let attribute_buffer_views = AttributeBufferViews {
			position: position_view,
			normal: normal_view,
			tangent: tangent_view,
			bitangent: None,
			color: None,
			texcoord_0: texcoord_0_view,
			texcoord_1: None,
			texcoord_2: None,
			texcoord_3: None,
		};
		(attribute_buffer, attribute_buffer_views)
	}
	fn create_index_buffer(
		&self,
		gl: &WebGl2RenderingContext,
	) -> (Rc<Buffer>, BufferView) {
		let indices = self.indices.as_ref().expect("Expected Some index data");

		let ratio = mem::size_of::<u32>() / mem::size_of::<u8>();
		let length = indices.len() * ratio;
		let ptr = indices.as_ptr() as *mut u8;

		let index_data = unsafe { std::slice::from_raw_parts(ptr, length) };

		let index_buffer = Rc::new(
			Buffer::new(
				gl,
				WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
				index_data,
			)
			.unwrap(),
		);

		let index_buffer_view = BufferView {
			buffer: 1,
			length: index_data.len(),
			buffer_offset: 0,
			offset: 0,
			combined_offset: 0,
			stride: 0,
			component_size: 1,
			component_count: indices.len() as i32,
			component_type: WebGl2RenderingContext::UNSIGNED_INT,
		};

		(index_buffer, index_buffer_view)
	}
}
