// File: src/lib.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use std::rc::Rc;

use rctree::Node;

use cgmath::{
	Matrix4,
	One,
};
use web_sys::WebGl2RenderingContext;

use crate::{
	model::{
		mesh::Mesh,
		Drawable,
	},
	program::Program,
};

#[derive(Debug)]
pub struct SceneGraph {
	pub root: Node<NodeData>,
}
impl Default for SceneGraph {
	fn default() -> Self {
		Self {
			root: Node::new(NodeData::default()),
		}
	}
}
impl Drawable for SceneGraph {
	fn draw(&self, gl: &WebGl2RenderingContext, program: &Program) {
		draw_node(&self.root, gl, program);
	}
}

pub fn draw_node(
	node: &Node<NodeData>,
	gl: &WebGl2RenderingContext,
	program: &Program,
) {
	{
		let node = node.borrow();
		if let NodeTypeData::Mesh(ref mesh) = node.node_type_data {
			let model_loc = program.uniform_locations.get("MODEL_MATRIX");
			gl.uniform_matrix4fv_with_f32_array(
				model_loc,
				false,
				&crate::mat_4_to_array(&node.transform.world_matrix),
			);
			mesh.draw(gl, program);

			// crate::log!("{:?}", mesh);
		}
	}
	for ref child in node.children() {
		draw_node(child, gl, program);
	}
}

pub fn set_local_matrix(
	node: &mut Node<NodeData>,
	local_matrix: &Matrix4<f32>,
) {
	node.borrow_mut().transform.local_matrix = *local_matrix;
	let world_matrix = match node.parent() {
		Some(ref parent) => {
			let parent_world_matrix = &parent.borrow().transform.world_matrix;
			parent_world_matrix * local_matrix
		}
		None => *local_matrix,
	};
	node.borrow_mut().transform.world_matrix = world_matrix;

	for ref mut child in node.children() {
		update_world_matrix(child, &world_matrix);
	}
}
fn update_world_matrix(
	node: &mut Node<NodeData>,
	parent_world_matrix: &Matrix4<f32>,
) {
	let world_matrix =
		parent_world_matrix * node.borrow_mut().transform.local_matrix;
	node.borrow_mut().transform.world_matrix = world_matrix;

	for ref mut child in node.children() {
		update_world_matrix(child, &world_matrix);
	}
}

#[derive(Debug, Clone)]
pub struct NodeData {
	transform: Transform,
	node_type_data: NodeTypeData,
}
impl Default for NodeData {
	fn default() -> Self {
		Self {
			transform: Transform::default(),
			node_type_data: NodeTypeData::default(),
		}
	}
}
impl NodeData {
	pub fn new(transform: Transform, node_type_data: NodeTypeData) -> Self {
		Self {
			transform,
			node_type_data,
		}
	}
	pub fn ref_transform(&self) -> &Transform {
		&self.transform
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Transform {
	local_matrix: Matrix4<f32>,
	world_matrix: Matrix4<f32>,
}
impl Default for Transform {
	fn default() -> Self {
		Self {
			local_matrix: Matrix4::one(),
			world_matrix: Matrix4::one(),
		}
	}
}
impl Transform {
	pub fn new(local_matrix: &Matrix4<f32>) -> Self {
		Self {
			local_matrix: *local_matrix,
			world_matrix: *local_matrix,
		}
	}
	pub fn local_matrix(&self) -> Matrix4<f32> {
		self.local_matrix
	}
	pub fn world_matrix(&self) -> Matrix4<f32> {
		self.local_matrix
	}
}

#[derive(Debug, Clone)]
pub enum NodeTypeData {
	Transform,
	Mesh(Rc<Mesh>),
}
impl NodeTypeData {
	pub fn is_transform(&self) -> bool {
		matches!(self, Self::Transform)
	}
	pub fn is_mesh(&self) -> bool {
		matches!(self, Self::Mesh(_))
	}
}
impl Default for NodeTypeData {
	fn default() -> Self {
		Self::Transform
	}
}
