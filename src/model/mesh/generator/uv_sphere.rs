// File: src/model/mesh/generator/uv_sphere.rs
// Author: Jacob Guenther
// Date created: April 2021
// License: AGPLv3
//
// Description:

use std::{
	f32::consts::PI,
	rc::Rc,
};

use cgmath::{
	Point3,
	Vector2,
	Vector3,
};

use web_sys::WebGl2RenderingContext;

use crate::model::mesh::Mesh;

use super::{
	vertex::Vertex,
	MeshGenerator,
	MeshMode,
};

pub fn generate_uv_sphere(
	gl: &WebGl2RenderingContext,
	radius: f32,
	vertical_subdivisions: usize,
	horizontal_subdivisions: usize,
) -> Rc<Mesh> {
	let vertex_count =
		(vertical_subdivisions + 1) * (horizontal_subdivisions + 1);
	let index_count = (vertical_subdivisions - 1) * horizontal_subdivisions * 6;
	let mut generator =
		MeshGenerator::new(Some(vertex_count), Some(index_count), None);

	let mut sector_angle_cos = Vec::with_capacity(horizontal_subdivisions + 1);
	let mut sector_angle_sin = Vec::with_capacity(horizontal_subdivisions + 1);

	let stack_count = vertical_subdivisions as u32;
	let sector_count = horizontal_subdivisions as u32;

	let stack_count_f = stack_count as f32;
	let sector_count_f = sector_count as f32;

	let radius_inverse = 1.0 / radius;
	let sector_step = 2.0 * PI / sector_count_f;
	let stack_step = PI / stack_count_f;

	let mut stack_angle = PI * 0.5;
	for stack in 0..(stack_count + 1) {
		let xz = radius * stack_angle.cos();
		let y = radius * stack_angle.sin();

		let mut sector_angle: f32 = 0.0;
		let texture_coord_t = stack as f32 / stack_count_f;
		for sector in 0..(sector_count + 1) {
			let (cos, sin) = if stack == 0 {
				let cos = sector_angle.cos();
				let sin = sector_angle.sin();
				sector_angle += sector_step;
				sector_angle_cos.push(cos);
				sector_angle_sin.push(sin);
				(cos, sin)
			} else {
				(
					sector_angle_cos[sector as usize],
					sector_angle_sin[sector as usize],
				)
			};

			let x = xz * cos;
			let z = xz * sin;
			let position = Point3::new(x, y, z);

			let nx = x * radius_inverse;
			let ny = y * radius_inverse;
			let nz = z * radius_inverse;
			let normal = Some(Vector3::new(nx, ny, nz));

			let texture_coord_s = sector as f32 / sector_count_f;
			let texcoord_0 =
				Some(Vector2::new(texture_coord_s, texture_coord_t));

			let vertex = Vertex {
				position,
				normal,
				texcoord_0,
				..Vertex::default()
			};
			generator.push_vertex(&vertex);
		}
		stack_angle -= stack_step;
	}

	for i in 0..stack_count {
		let mut k1 = i * (sector_count + 1);
		let mut k2 = k1 + sector_count + 1;

		for _j in 0..sector_count {
			if i != 0 {
				generator.push_index_slice(&[k1, k2, k1 + 1])
			}

			if i != (stack_count - 1) {
				generator.push_index_slice(&[k1 + 1, k2, k2 + 1])
			}

			k1 += 1;
			k2 += 1;
		}
	}

	generator.generate_mesh(gl, MeshMode::IndexedTriangles)
}
