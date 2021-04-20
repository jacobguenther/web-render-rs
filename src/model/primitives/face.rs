// File: src/model/primitives/face.rs
// Author: Jacob Guenther
// Date created: April 2021
// License: AGPLv3
//
// Description:

use super::vertex::Vertex;
use cgmath::{InnerSpace, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Face {
	vertex_0_index: usize,
	vertex_1_index: usize,
	vertex_2_index: usize,

	area: f32,
	normal: Vector3<f32>,
	tangent: Vector3<f32>,
	bitangent: Vector3<f32>,
}
impl Face {
	pub fn new(
		vertices: &[Vertex],
		vertex_0_index: usize,
		vertex_1_index: usize,
		vertex_2_index: usize,
	) -> Self {
		let (area, normal, tangent, bitangent) = Self::area_normal_tangent_bitangent(
			vertices,
			vertex_0_index,
			vertex_1_index,
			vertex_2_index,
		);
		Self {
			vertex_0_index,
			vertex_1_index,
			vertex_2_index,
			area,
			normal,
			tangent,
			bitangent,
		}
	}
	fn area_normal_tangent_bitangent(
		vertices: &[Vertex],
		vertex_0_index: usize,
		vertex_1_index: usize,
		vertex_2_index: usize,
	) -> (f32, Vector3<f32>, Vector3<f32>, Vector3<f32>) {
		let p0 = vertices[vertex_0_index].position;
		let p1 = vertices[vertex_1_index].position;
		let p2 = vertices[vertex_2_index].position;
		let v01 = p1 - p0;
		let v02 = p2 - p0;
		let n = v01.cross(v02);
		let area = n.magnitude() * 0.5;
		let b = n.cross(v01).normalize();
		(area, n.normalize(), v01.normalize(), b)
	}
}
