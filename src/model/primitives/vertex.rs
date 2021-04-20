// File: src/model/primitives/vertex.rs
// Author: Jacob Guenther
// Date created: April 2021
// License: AGPLv3
//
// Description:

use cgmath::{Point3, Vector2, Vector3};

#[derive(Clone, Debug)]
pub struct Vertex {
	pub adjacent_faces: Vec<usize>,
	pub position: Point3<f32>,
	pub normal: Option<Vector3<f32>>,
	pub tangent: Option<Vector3<f32>>,
	pub bitangent: Option<Vector3<f32>>,
	pub color: Option<Vector3<f32>>,
	pub texcoord_0: Option<Vector2<f32>>,
	pub texcoord_1: Option<Vector2<f32>>,
	pub texcoord_2: Option<Vector2<f32>>,
	pub texcoord_3: Option<Vector2<f32>>,
}
impl Vertex {
	pub fn as_slice(&self) -> [f32; 3] {
		[self.position.x, self.position.y, self.position.z]
	}
}
impl Vertex {
	pub const fn stride() -> i32 {
		let size_of_position =
			Vertex::position_component_count() * Vertex::position_component_size();
		let size_of_normal = 0;
		size_of_position + size_of_normal
	}
	pub const fn position_component_size() -> i32 {
		4
	}
	pub const fn position_component_count() -> i32 {
		3
	}
}
