// File: src/model/terrain.rs
// Author: Jacob Guenther
// Date created: April 2021
// License: AGPLv3
//
// Description:

use std::{mem, rc::Rc};

use crate::program::Program;

use super::{
	buffer::Buffer,
	buffer_view::BufferView,
	material::{Material, MetallicRoughnessPart, OcclusionPart},
	mesh::{AttributeBufferViews, Mesh},
	primitives::{face::Face, vertex::Vertex},
	Drawable,
};
use cgmath::{Point3, Vector3};
use web_sys::WebGl2RenderingContext;

impl Drawable for Terrain {
	fn draw(&self, gl: &WebGl2RenderingContext, program: &Program) {
		self.mesh.draw(gl, program);
	}
}

pub struct Terrain {
	chunk_size: usize,
	scale: Vector3<f32>,

	mesh: Mesh,
	// indices: Vec<u32>,
	// vertices: Vec<Vertex>,
}

impl Terrain {
	pub fn chunk_size(&self) -> usize {
		self.chunk_size
	}
	pub fn scale(&self) -> &Vector3<f32> {
		&self.scale
	}
	pub fn generate(
		gl: &WebGl2RenderingContext,
		chunk_size: usize,
		scale: &Vector3<f32>,
		heights: &[&[f32]],
	) -> Result<Self, &'static str> {
		if chunk_size == 0 {
			return Err("Cannot have a terrain of size 0");
		} else if scale.x <= 0.0 || scale.y <= 0.0 || scale.z <= 0.0 {
			return Err("Scale must be positive");
		}

		let vertex_count = (chunk_size + 1) * (chunk_size + 1);
		let mut vertices = Vec::with_capacity(vertex_count);
		Self::_init_vertices_with_positions(&mut vertices, chunk_size, &scale, heights);
		let (indices, faces) = Self::_setup_indices_and_faces(&vertices, chunk_size);
		// crate::log!("faces: {:#?}", faces);

		Self::_generate_normals(&mut vertices, &faces);
		// Self::_generate_tangents(&mut vertices);
		// Self::_generate_bitangents(&mut vertices);
		// Self::_generate_color(&mut vertices);
		// Self::_generate_texcoords(&mut vertices);
		// Self::_setup_indices();
		// Self::_setup_faces();

		let vertex_data = Self::_setup_vertices_data(&vertices);
		let attribute_buffer = Rc::new(Buffer::new(
			gl,
			WebGl2RenderingContext::ARRAY_BUFFER,
			vertex_data,
		)?);

		let mut buffers = vec![attribute_buffer];

		let position_view = {
			BufferView {
				buffer: 0,
				length: std::mem::size_of::<f32>()
					* vertices.len() * Vertex::position_component_count() as usize,
				buffer_offset: 0,
				offset: 0,
				combined_offset: 0,
				stride: 0,
				component_size: 3,
				component_count: vertices.len() as i32,
				component_type: WebGl2RenderingContext::FLOAT,
			}
		};

		let views = AttributeBufferViews {
			position: position_view,
			normal: None,
			tangent: None,
			bitangent: None,
			color: None,
			texcoord_0: None,
			texcoord_1: None,
			texcoord_2: None,
			texcoord_3: None,
		};

		let index_view: Option<BufferView> = {
			Some(BufferView {
				buffer: 1,
				length: 4 * indices.len(),
				buffer_offset: 0,
				offset: 0,
				combined_offset: 0,
				stride: 0,
				component_size: 1,
				component_count: indices.len() as i32,
				component_type: WebGl2RenderingContext::UNSIGNED_INT,
			})
		};
		let index_data = Self::_setup_indices_data(&indices);
		let index_buffer = Rc::new(Buffer::new(
			gl,
			WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
			index_data,
		)?);
		buffers.push(index_buffer);

		let material = Rc::new(Material {
			diffuse_tex: None,
			normal_tex: None,
			metallic_roughness_part: MetallicRoughnessPart::default(),
			occlusion_part: OcclusionPart::default(),
		});
		let mesh = Mesh::new(gl, &material, &buffers, &index_view, &views)?;
		Ok(Self {
			chunk_size,
			scale: *scale,
			mesh,
			// indices,
			// vertices,
		})
	}
	fn _init_vertices_with_positions(
		vertices: &mut Vec<Vertex>,
		chunk_size: usize,
		scale: &Vector3<f32>,
		heights: &[&[f32]],
	) {
		let chunk_size_p1 = chunk_size + 1;
		let chunk_size_m2 = chunk_size * 2;
		let chunk_size_2 = chunk_size * chunk_size;

		for z in 0..(chunk_size_p1) {
			for x in 0..(chunk_size_p1) {
				let mut adjacent_faces: Vec<usize> = Vec::new();
				match (x, z) {
					(0, 0) => {
						// corner: lower left
						adjacent_faces.push(0);
					}
					(x, 0) if x == chunk_size => {
						// corner: lower right
						let start = chunk_size + (chunk_size / 2);
						adjacent_faces.push(start + 1);
						adjacent_faces.push(start + 2);
					}
					(0, z) if z == chunk_size => {
						// corner: upper left
						let start = chunk_size_m2 * (chunk_size - 1);
						adjacent_faces.push(start);
						adjacent_faces.push(start + 1);
					}
					(x, z) if x == chunk_size && z == chunk_size => {
						// corner: upper right
						adjacent_faces.push(chunk_size_2 * 2 - 1);
					}
					(x, 0) if x < chunk_size => {
						// side: bottom
						let start = x * 2;
						adjacent_faces.push(start - 2);
						adjacent_faces.push(start - 1);
						adjacent_faces.push(start);
					}
					(x, z) if z == chunk_size && x < chunk_size => {
						// side: top
						let start = chunk_size_m2 * (z - 1) + x * 2;
						adjacent_faces.push(start - 1);
						adjacent_faces.push(start);
						adjacent_faces.push(start + 1);
					}
					(0, z) if z < chunk_size => {
						// side: left
						let start = chunk_size_m2 * (z - 1);
						adjacent_faces.push(start);
						adjacent_faces.push(start + 1);
						adjacent_faces.push(start + chunk_size_m2);
					}
					(x, z) if x == chunk_size && z < chunk_size => {
						// side: right
						let start = chunk_size_m2 * z - 1;
						adjacent_faces.push(start);
						adjacent_faces.push(start + chunk_size_m2 - 1);
						adjacent_faces.push(start + chunk_size_m2);
					}
					(x, z) => {
						// center
						let start_below = chunk_size_m2 * (z - 1) + (x - 1) * 2;
						adjacent_faces.push(start_below + 1);
						adjacent_faces.push(start_below + 2);
						adjacent_faces.push(start_below + 3);
						let start_above = start_below + chunk_size_m2;
						adjacent_faces.push(start_above);
						adjacent_faces.push(start_above + 1);
						adjacent_faces.push(start_above + 2);
					}
				}

				let height = heights[z as usize][x as usize];

				let x = x as f32 / scale.x;
				let y = height / scale.y;
				let z = z as f32 / scale.z;

				vertices.push(Vertex {
					adjacent_faces,
					position: Point3::new(x, y, z),
					normal: None,
					tangent: None,
					bitangent: None,
					color: None,
					texcoord_0: None,
					texcoord_1: None,
					texcoord_2: None,
					texcoord_3: None,
				});
			}
		}
	}
	fn _generate_normals(vertices: &mut Vec<Vertex>, _faces: &[Face]) {
		vertices.iter_mut().for_each(|_vertex| {});
	}
	fn _setup_vertices_data(vertices: &[Vertex]) -> &[u8] {
		let mut data_f32 = Vec::new();
		for v in vertices.iter() {
			data_f32.extend_from_slice(&v.as_slice());
		}

		let ratio = mem::size_of::<f32>() / mem::size_of::<u8>();

		let length = data_f32.len() * ratio;
		let ptr = data_f32.as_ptr() as *mut u8;

		unsafe { std::slice::from_raw_parts(ptr, length) }
	}
	fn _setup_indices_and_faces(vertices: &[Vertex], chunk_size: usize) -> (Vec<u32>, Vec<Face>) {
		let face_count = chunk_size * chunk_size * 2;
		let mut indices = Vec::with_capacity(face_count * 3);
		let mut faces = Vec::with_capacity(face_count);
		for x in 0..(chunk_size) {
			let mut lower_left = x * (chunk_size + 1);
			let mut lower_right = lower_left + 1;
			let mut upper_left = lower_left + chunk_size + 1;
			let mut upper_right = upper_left + 1;
			for _z in 0..(chunk_size) {
				indices.push(lower_left as u32);
				indices.push(lower_right as u32);
				indices.push(upper_left as u32);

				indices.push(lower_right as u32);
				indices.push(upper_right as u32);
				indices.push(upper_left as u32);

				faces.push(Face::new(vertices, lower_left, lower_right, upper_left));
				faces.push(Face::new(vertices, lower_right, upper_right, upper_left));

				lower_left = lower_right;
				lower_right += 1;
				upper_left = upper_right;
				upper_right += 1;
			}
		}
		(indices, faces)
	}
	fn _setup_indices_data(indices: &[u32]) -> &[u8] {
		let ratio = mem::size_of::<u32>() / mem::size_of::<u8>();

		let length = indices.len() * ratio;
		let ptr = indices.as_ptr() as *mut u8;

		unsafe { std::slice::from_raw_parts(ptr, length) }
	}
}
