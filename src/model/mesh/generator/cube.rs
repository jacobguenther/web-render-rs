// File: src/model/mesh/generator/cube.rs
// Author: Jacob Guenther
// Date created: April 2021
// License: AGPLv3
//
// Description:

use std::rc::Rc;

use cgmath::Point3;
use web_sys::WebGl2RenderingContext;

use crate::model::mesh::Mesh;

use super::{
	vertex::Vertex,
	MeshGenerator,
	MeshMode,
};

pub fn generate_cube(gl: &WebGl2RenderingContext) -> Rc<Mesh> {
	let vertex_count = 8;
	let index_count = 36;
	let face_count = 12;
	let mut generator = MeshGenerator::new(
		Some(vertex_count),
		Some(index_count),
		Some(face_count),
	);

	generator.push_vertex(&Vertex::new(Point3::new(-0.5, -0.5, -0.5)));
	generator.push_vertex(&Vertex::new(Point3::new(0.5, -0.5, -0.5)));
	generator.push_vertex(&Vertex::new(Point3::new(0.5, -0.5, 0.5)));
	generator.push_vertex(&Vertex::new(Point3::new(-0.5, -0.5, 0.5)));
	generator.push_vertex(&Vertex::new(Point3::new(-0.5, 0.5, -0.5)));
	generator.push_vertex(&Vertex::new(Point3::new(0.5, 0.5, -0.5)));
	generator.push_vertex(&Vertex::new(Point3::new(0.5, 0.5, 0.5)));
	generator.push_vertex(&Vertex::new(Point3::new(-0.5, 0.5, 0.5)));

	{
		let indices = generator.mut_ref_indices();
		indices.extend_from_slice(&[
			0, 1, 5, //
			5, 4, 0, //
			1, 2, 6, //
			6, 5, 1, //
			2, 3, 7, //
			7, 6, 2, //
			3, 0, 4, //
			4, 7, 3, //
			0, 1, 2, //
			2, 3, 0, //
			4, 5, 6, //
			6, 7, 4,
		]);
	}

	generator.generate_mesh(gl, MeshMode::IndexedTriangles)
}
