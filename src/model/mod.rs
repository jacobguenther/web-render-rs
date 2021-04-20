// File: src/model/mod.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

pub mod buffer;
pub mod buffer_view;
pub mod material;
pub mod mesh;
pub mod primitives;
pub mod terrain;
pub mod texture;

use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

use self::mesh::Mesh;
use crate::program::Program;

pub trait Drawable {
	fn draw(&self, gl: &WebGl2RenderingContext, program: &Program);
}

#[derive(Clone, Debug)]
pub struct Model {
	pub meshes: Vec<Rc<Mesh>>,
}
impl Model {
	pub fn new(meshes: &[Rc<Mesh>]) -> Self {
		Self {
			meshes: meshes.to_owned(),
		}
	}
}
impl Drawable for Model {
	fn draw(&self, gl: &WebGl2RenderingContext, program: &Program) {
		self.meshes.iter().for_each(|mesh| mesh.draw(gl, program));
	}
}
