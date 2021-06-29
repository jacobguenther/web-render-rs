// File: src/model/terrain.rs
// Author: Jacob Guenther
// Date created: April 2021
// License: AGPLv3
//
// Description:

use std::rc::Rc;

use crate::{
	model::Drawable,
	program::Program,
};

use super::super::Mesh;
use super::{
	vertex::Vertex,
	*,
};

use cgmath::{
	Point3,
	Vector3,
};
use web_sys::WebGl2RenderingContext;

impl Drawable for Terrain {
	fn draw(&self, gl: &WebGl2RenderingContext, program: &Program) {
		self.mesh.draw(gl, program);
	}
}

pub struct Terrain {
	chunk_size: usize,
	scale: Vector3<f32>,

	mesh: Rc<Mesh>,
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
		heights: &[f32],
	) -> Result<Self, &'static str> {
		if chunk_size == 0 {
			return Err("Cannot have a terrain of size 0");
		} else if scale.x <= 0.0 || scale.y <= 0.0 || scale.z <= 0.0 {
			return Err("Scale must be positive");
		}

		let vertex_count = (chunk_size + 1) * (chunk_size + 1);
		let face_count = chunk_size * chunk_size * 2;
		let index_count = face_count * 3;

		let mut generator = MeshGenerator::new(
			Some(vertex_count),
			Some(index_count),
			Some(face_count),
		);
		{
			let vertices = generator.mut_ref_vertices();
			Self::_init_vertices_with_positions(
				vertices, chunk_size, &scale, heights,
			);
		}
		{
			let indices = generator.mut_ref_indices();
			Self::_init_indices(indices, chunk_size);
		}
		{
			let vertex_face_adjacency_list =
				generator.mut_ref_vertex_face_adjacency_list();
			*vertex_face_adjacency_list =
				Some(Vec::with_capacity(vertex_count));
			let vertex_face_adjacency_list =
				vertex_face_adjacency_list.as_mut().unwrap();
			Self::_init_vertex_face_adjacency_list(
				vertex_face_adjacency_list,
				chunk_size,
			);
		}
		let mesh = generator.generate_mesh(gl, MeshMode::IndexedTriangles);

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
		heights: &[f32],
	) {
		let mut i = 0;
		for z in 0..(chunk_size + 1) {
			let z = z as f32 * scale.z;
			for x in 0..(chunk_size + 1) {
				let x = x as f32 * scale.x;

				let height = heights[i];
				i += 1;
				let y = height * scale.y;

				vertices.push(Vertex {
					position: Point3::new(x, y, z),
					normal: None,
					tangent: None,
					// bitangent: None,
					// color: None,
					texcoord_0: None,
					// texcoord_1: None,
					// texcoord_2: None,
					// texcoord_3: None,
				});
			}
		}
	}
	fn _init_vertex_face_adjacency_list(
		vertex_face_adjacency_list: &mut Vec<Vec<usize>>,
		chunk_size: usize,
	) {
		let chunk_size_p1 = chunk_size + 1;
		let chunk_size_m2 = chunk_size * 2;
		let chunk_size_2 = chunk_size * chunk_size;

		for z in 0..(chunk_size_p1) {
			for x in 0..(chunk_size_p1) {
				let adjacent_faces = match (x, z) {
					(0, 0) => {
						// corner: lower left
						vec![0]
					}
					(x, 0) if x == chunk_size => {
						// corner: lower right
						let start = chunk_size + (chunk_size / 2);
						vec![start + 1, start + 2]
					}
					(0, z) if z == chunk_size => {
						// corner: upper left
						let start = chunk_size_m2 * (chunk_size - 1);
						vec![start, start + 1]
					}
					(x, z) if x == chunk_size && z == chunk_size => {
						// corner: upper right
						vec![chunk_size_2 * 2 - 1]
					}
					(x, 0) if x < chunk_size => {
						// side: bottom
						let start = x * 2;
						vec![start - 2, start - 1, start]
					}
					(x, z) if z == chunk_size && x < chunk_size => {
						// side: top
						let start = chunk_size_m2 * (z - 1) + x * 2;
						vec![start - 1, start, start + 1]
					}
					(0, z) if z < chunk_size => {
						// side: left
						let start = chunk_size_m2 * (z - 1);
						vec![start, start + 1, start + chunk_size_m2]
					}
					(x, z) if x == chunk_size && z < chunk_size => {
						// side: right
						let start = chunk_size_m2 * z - 1;
						vec![
							start,
							start + chunk_size_m2 - 1,
							start + chunk_size_m2,
						]
					}
					(x, z) => {
						// center
						let start_below = chunk_size_m2 * (z - 1) + (x - 1) * 2;
						let start_above = start_below + chunk_size_m2;
						vec![
							start_below + 1,
							start_below + 2,
							start_below + 3,
							start_above,
							start_above + 1,
							start_above + 2,
						]
					}
				};
				vertex_face_adjacency_list.push(adjacent_faces);
			}
		}
	}
	fn _init_indices(indices: &mut Vec<u32>, chunk_size: usize) {
		for x in 0..chunk_size {
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

				lower_left = lower_right;
				lower_right += 1;
				upper_left = upper_right;
				upper_right += 1;
			}
		}
	}
}
