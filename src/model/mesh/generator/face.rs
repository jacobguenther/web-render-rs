// File: src/model/primitives/face.rs
// Author: Jacob Guenther
// Date created: April 2021
// License: AGPLv3
//
// Description:

use super::vertex::Vertex;
use cgmath::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Face {
	pub vertex_0_index: usize,
	pub vertex_1_index: usize,
	pub vertex_2_index: usize,

	pub normal: Vector3<f32>,
	// pub area: f32,
	// pub unit_normal: Vector3<f32>,
	pub tangent: Vector3<f32>,
	// pub bitangent: Vector3<f32>,
}
impl Face {
	pub fn new(
		vertices: &[Vertex],
		vertex_0_index: usize,
		vertex_1_index: usize,
		vertex_2_index: usize,
	) -> Self {
		let (normal, tangent) = Self::calc_normal(
			vertices,
			vertex_0_index,
			vertex_1_index,
			vertex_2_index,
		);

		// let area = normal.magnitude();
		// let unit_normal = normal.normalize();

		Self {
			vertex_0_index,
			vertex_1_index,
			vertex_2_index,

			normal,
			// area,
			// unit_normal,
			tangent,
			// bitangent,
		}
	}
	fn calc_normal(
		vertices: &[Vertex],
		vertex_0_index: usize,
		vertex_1_index: usize,
		vertex_2_index: usize,
	) -> (Vector3<f32>, Vector3<f32>) {
		let p0 = &vertices[vertex_0_index].position;
		let p1 = &vertices[vertex_1_index].position;
		let p2 = &vertices[vertex_2_index].position;
		let v01 = p1 - p0;
		let mut v02 = p2 - p0;
		let n = v02.cross(v01);
		let t = if v01.x > (0.0 + 0.05) {
			v01
		} else {
			v02.x = -v02.x;
			v02
		};
		(n, t)
	}
}
