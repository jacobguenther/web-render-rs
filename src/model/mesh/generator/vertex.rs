// File: src/model/primitives/vertex.rs
// Author: Jacob Guenther
// Date created: April 2021
// License: AGPLv3
//
// Description:

use cgmath::{
	Point3,
	Vector2,
	Vector3,
};
use web_sys::WebGl2RenderingContext;

use crate::model::buffer_view::BufferView;
// use cgmath::Vector2;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	pub position: Point3<f32>,
	pub normal: Option<Vector3<f32>>,
	pub tangent: Option<Vector3<f32>>,
	// pub bitangent: Option<Vector3<f32>>,
	// pub color: Option<Vector3<f32>>,
	pub texcoord_0: Option<Vector2<f32>>,
	// pub texcoord_1: Option<Vector2<f32>>,
	// pub texcoord_2: Option<Vector2<f32>>,
	// pub texcoord_3: Option<Vector2<f32>>,
}
impl Vertex {
	pub fn new(position: Point3<f32>) -> Self {
		Self {
			position,
			..Vertex::default()
		}
	}
	pub fn new_with_normal(
		position: Point3<f32>,
		normal: Vector3<f32>,
	) -> Self {
		Self {
			position,
			normal: Some(normal),
			..Vertex::default()
		}
	}
	pub fn as_slice(&self) -> Vec<f32> {
		let mut vertex_data =
			vec![self.position.x, self.position.y, self.position.z];
		if let Some(ref normal) = self.normal {
			vertex_data.push(normal.x);
			vertex_data.push(normal.y);
			vertex_data.push(normal.z);
		}
		if let Some(ref tangent) = self.tangent {
			vertex_data.push(tangent.x);
			vertex_data.push(tangent.y);
			vertex_data.push(tangent.z);
		}
		if let Some(ref texcoord_0) = self.texcoord_0 {
			vertex_data.push(texcoord_0.x);
			vertex_data.push(texcoord_0.y);
		}
		vertex_data
	}
}
impl Default for Vertex {
	fn default() -> Self {
		Self {
			position: Point3::new(0.0, 0.0, 0.0),
			normal: None,
			tangent: None,
			texcoord_0: None,
		}
	}
}
impl Vertex {
	const fn position_size() -> i32 {
		12
	}
	const fn normal_size() -> i32 {
		12
	}
	const fn tangent_size() -> i32 {
		12
	}
	const fn texcoord_size() -> i32 {
		8
	}

	pub fn stride(&self) -> i32 {
		Vertex::position_size()
			+ self.normal.map_or(0, |_n| Vertex::normal_size())
			+ self.tangent.map_or(0, |_t| Vertex::tangent_size())
			+ self.texcoord_0.map_or(0, |_t| Vertex::texcoord_size())
	}

	pub fn position_view(&self, vertex_count: usize) -> BufferView {
		self.view_helper(vertex_count, 0, 3)
	}
	pub fn normal_view(&self, vertex_count: usize) -> Option<BufferView> {
		if self.normal.is_some() {
			let normal_offset = Vertex::position_size();
			Some(self.view_helper(vertex_count, normal_offset, 3))
		} else {
			None
		}
	}
	pub fn tangent_view(&self, vertex_count: usize) -> Option<BufferView> {
		if self.tangent.is_some() {
			let tangent_offset = Vertex::position_size()
				+ self.normal.map_or(0, |_n| Vertex::normal_size());
			Some(self.view_helper(vertex_count, tangent_offset, 3))
		} else {
			None
		}
	}
	pub fn texcoord_0_view(&self, vertex_count: usize) -> Option<BufferView> {
		if self.texcoord_0.is_some() {
			let texcoord_0_offset = Vertex::position_size()
				+ self.normal.map_or(0, |_n| Vertex::normal_size())
				+ self.tangent.map_or(0, |_n| Vertex::tangent_size());
			Some(self.view_helper(vertex_count, texcoord_0_offset, 2))
		} else {
			None
		}
	}

	fn view_helper(
		&self,
		vertex_count: usize,
		offset: i32,
		component_size: i32,
	) -> BufferView {
		let stride = self.stride();
		BufferView {
			buffer: 0,
			length: vertex_count * stride as usize,
			buffer_offset: 0,
			offset,
			combined_offset: offset,
			stride,
			component_size,
			component_count: vertex_count as i32,
			component_type: WebGl2RenderingContext::FLOAT,
		}
	}
}
