// File: src/model/mesh/generator/icosphere.rs
// Author: Jacob Guenther
// Date created: April 2021
// License: AGPLv3
//
// Description:

use std::{
	collections::HashMap,
	f32::consts::PI,
	rc::Rc,
};

use cgmath::{
	EuclideanSpace,
	InnerSpace,
	Point2,
	Point3,
	Vector2,
	Vector3,
	Zero,
};

use web_sys::WebGl2RenderingContext;

use crate::model::mesh::Mesh;

use super::{
	vertex::Vertex,
	MeshGenerator,
	MeshMode,
};

type Vertices = Vec<Vertex>;
type Indices = Vec<u32>;
type SharedIndicesKey = ([u8; 4], [u8; 4]);
type SharedIndices = HashMap<SharedIndicesKey, usize>;

pub fn generate_icosphere(
	gl: &WebGl2RenderingContext,
	radius: f32,
	subdivisions: usize,
) -> Rc<Mesh> {
	let (mut vertices, mut indices, mut shared_indices) =
		icosahedron_smooth_normals(radius);

	for _i in 1..(subdivisions + 1) {
		let temp_indices = indices.clone();

		indices.clear();

		let index_count = temp_indices.len();
		for j in (0..index_count).step_by(3) {
			let i1 = temp_indices[j];
			let i2 = temp_indices[j + 1];
			let i3 = temp_indices[j + 2];

			let v1 = vertices[i1 as usize];
			let v2 = vertices[i2 as usize];
			let v3 = vertices[i3 as usize];

			let new_v1 = compute_half_vertex(&v1, &v2, radius);
			let new_v2 = compute_half_vertex(&v2, &v3, radius);
			let new_v3 = compute_half_vertex(&v1, &v3, radius);

			let new_i1 =
				subvertex_attribs(&mut vertices, &mut shared_indices, &new_v1);
			let new_i2 =
				subvertex_attribs(&mut vertices, &mut shared_indices, &new_v2);
			let new_i3 =
				subvertex_attribs(&mut vertices, &mut shared_indices, &new_v3);

			indices.extend_from_slice(&[i1, new_i1, new_i3]);
			indices.extend_from_slice(&[new_i1, i2, new_i2]);
			indices.extend_from_slice(&[new_i1, new_i2, new_i3]);
			indices.extend_from_slice(&[new_i3, new_i2, i3]);
		}
	}
	let mut generator = MeshGenerator::new(None, None, None);
	generator.generate_mesh(gl, MeshMode::IndexedTriangles)
}

pub fn icosahedron_smooth_normals(
	radius: f32,
) -> (Vertices, Indices, SharedIndices) {
	// smooth icosahedron has 14 non-shared (0 to 13) and
	// 8 shared vertices (14 to 21) (total 22 vertices)
	//  00  01  02  03  04          //
	//  /\  /\  /\  /\  /\          //
	// /  \/  \/  \/  \/  \         //
	//10--14--15--16--17--11        //
	// \  /\  /\  /\  /\  /\        //
	//  \/  \/  \/  \/  \/  \       //
	//  12--18--19--20--21--13      //
	//   \  /\  /\  /\  /\  /       //
	//    \/  \/  \/  \/  \/        //
	//    05  06  07  08  09        //

	// let vertex_count = 10 * subdivisions + 2;
	// let face_count = 20 * subdivisions;

	let s_step = 186.0 / 2048.0; // horizontal texture step
	let t_step = 322.0 / 1024.0; // vertical texture step

	let temp_vertices = icosahedron_vertices(radius);
	let mut vertices = Vec::with_capacity(22);

	// add top
	let mut v0 = temp_vertices[0];
	v0.texcoord_0 = Some(Vector2::new(s_step, 0.0));
	vertices.push(v0);

	let mut v1 = v0;
	v1.texcoord_0 = Some(Vector2::new(s_step * 3.0, 0.0));
	vertices.push(v1);

	let mut v2 = v0;
	v2.texcoord_0 = Some(Vector2::new(s_step * 5.0, 0.0));
	vertices.push(v2);

	let mut v3 = v0;
	v3.texcoord_0 = Some(Vector2::new(s_step * 7.0, 0.0));
	vertices.push(v3);

	let mut v4 = v0;
	v4.texcoord_0 = Some(Vector2::new(s_step * 9.0, 0.0));
	vertices.push(v4);

	// add bottom
	let mut v5 = temp_vertices[11];
	v5.texcoord_0 = Some(Vector2::new(s_step * 2.0, t_step * 3.0));
	vertices.push(v5);

	let mut v6 = v5;
	v6.texcoord_0 = Some(Vector2::new(s_step * 4.0, t_step * 3.0));
	vertices.push(v6);

	let mut v7 = v5;
	v7.texcoord_0 = Some(Vector2::new(s_step * 6.0, t_step * 3.0));
	vertices.push(v7);

	let mut v8 = v5;
	v8.texcoord_0 = Some(Vector2::new(s_step * 8.0, t_step * 3.0));
	vertices.push(v8);

	let mut v9 = v5;
	v9.texcoord_0 = Some(Vector2::new(s_step * 10.0, t_step * 3.0));
	vertices.push(v9);

	// duplicated edge vertices
	let mut v10 = temp_vertices[1];
	v10.texcoord_0 = Some(Vector2::new(0.0, t_step));
	vertices.push(v10);

	let mut v11 = v10;
	v11.texcoord_0 = Some(Vector2::new(s_step * 10.0, t_step));
	vertices.push(v11);

	let mut v12 = temp_vertices[6];
	v12.texcoord_0 = Some(Vector2::new(s_step, t_step * 2.0));
	vertices.push(v12);

	let mut v13 = v12;
	v13.texcoord_0 = Some(Vector2::new(s_step * 11.0, t_step * 2.0));
	vertices.push(v13);

	let mut shared_indices = HashMap::new();
	// shared vertices
	let mut s_steps = 2.0;
	let mut t_steps = 1.0;
	let mut on_upper_row = true;
	for i in 2..11 {
		if i == 6 {
			on_upper_row = false;
			s_steps = 3.0;
			t_steps = 2.0;
			continue;
		}
		let mut v = temp_vertices[i];
		let s_coord = s_step * s_steps;
		let t_coord = t_step * t_steps;
		v.texcoord_0 = Some(Vector2::new(s_coord, t_coord));
		vertices.push(v);

		let key = make_key(s_coord, t_coord);
		shared_indices.insert(key, vertices.len() - 1);

		if on_upper_row {
			s_steps += 2.0;
		} else {
			t_steps += 2.0;
		}
	}
	let indices = vec![
		0, 10, 14, // top
		1, 14, 15, //
		2, 15, 16, //
		3, 16, 17, //
		4, 17, 11, //
		10, 12, 14, // middle
		12, 18, 14, //
		14, 18, 15, //
		18, 19, 15, //
		15, 19, 16, //
		19, 20, 16, //
		16, 20, 17, //
		20, 21, 17, //
		17, 21, 11, //
		21, 13, 11, //
		5, 18, 12, // bottom
		6, 19, 18, //
		7, 20, 19, //
		8, 21, 20, //
		9, 13, 21, //
	];

	(vertices, indices, shared_indices)
}
fn make_key(s: f32, t: f32) -> SharedIndicesKey {
	(s.to_bits().to_be_bytes(), t.to_bits().to_be_bytes())
}
fn get_coords_from_key(key: &SharedIndicesKey) -> Vector2<f32> {
	Vector2::new(
		f32::from_bits(u32::from_be_bytes(key.0)),
		f32::from_bits(u32::from_be_bytes(key.1)),
	)
}

fn icosahedron_vertices(radius: f32) -> Vertices {
	//  0   0   0   0   0           //
	//  /\  /\  /\  /\  /\          //
	// /  \/  \/  \/  \/  \         //
	//1---2---3---4---5---1         //
	// \  /\  /\  /\  /\  /\        //
	//  \/  \/  \/  \/  \/  \       //
	//  6---7---8---9---10---6      //
	//   \  /\  /\  /\  /\  /       //
	//    \/  \/  \/  \/  \/        //
	//    11  11  11  11  11        //

	let vertex_count = 12;
	let mut vertices = Vec::with_capacity(vertex_count);

	vertices.push(Vertex::new_with_normal(
		Point3::new(0.0, radius, 0.0),
		Vector3::new(0.0, 1.0, 0.0),
	));

	let h_angle = PI / 180.0 * 72.0;
	let v_angle = ((1.0 / 2.0) as f32).atan();
	let sin_v_angle = v_angle.sin();
	let cos_v_angle = v_angle.sin();

	let mut h_angle_2 = -PI / 2.0;
	let mut h_angle_1 = h_angle_2 - h_angle / 2.0;

	let y = radius * sin_v_angle;
	let xz = radius * cos_v_angle;

	let mut lower_vertices = Vec::with_capacity(5);
	for _i in 1..6 {
		let sin_h_angle_2 = h_angle_2.sin();
		let cos_h_angle_2 = h_angle_2.cos();

		let sin_h_angle_1 = h_angle_1.sin();
		let cos_h_angle_1 = h_angle_1.cos();

		let x1 = xz * cos_h_angle_1;
		let x2 = xz * cos_h_angle_2;

		let z1 = xz * sin_h_angle_1;
		let z2 = xz * sin_h_angle_2;

		h_angle_1 += h_angle;
		h_angle_2 += h_angle;

		let position = Point3::new(x1, y, z1);
		let normal_top = position.to_vec().normalize();
		vertices.push(Vertex::new_with_normal(position, normal_top));

		lower_vertices.push(Vertex::new_with_normal(
			Point3::new(x2, -y, z2),
			Vector3::new(normal_top.x, -normal_top.y, normal_top.z),
		));
	}
	vertices.extend_from_slice(&lower_vertices);

	// bottom
	vertices.push(Vertex::new(Point3::new(0.0, -radius, 0.0)));

	vertices
}

fn compute_half_vertex(a: &Vertex, b: &Vertex, radius: f32) -> Vertex {
	let a_b_midpoint = a.position.midpoint(b.position);
	let new_normal = a_b_midpoint.to_vec().normalize();
	let new_position = Point3::from_vec(new_normal * radius);
	let new_texture_coord = if let (Some(a_tex_coord), Some(b_tex_coord)) =
		(a.texcoord_0, b.texcoord_0)
	{
		Some(
			Point2::from_vec(a_tex_coord)
				.midpoint(Point2::from_vec(b_tex_coord))
				.to_vec(),
		)
	} else {
		Some(Vector2::zero())
	};
	Vertex {
		position: new_position,
		normal: Some(new_normal),
		tangent: None,
		texcoord_0: new_texture_coord,
	}
}
fn subvertex_attribs(
	vertices: &mut Vec<Vertex>,
	shared_indices: &mut SharedIndices,
	vertex: &Vertex,
) -> u32 {
	if is_shared_texture_coord(shared_indices, vertex) {
		0
	} else {
		vertices.push(*vertex);
		(vertices.len() - 1) as u32
	}
}
fn is_shared_texture_coord(
	shared_indices: &SharedIndices,
	vertex: &Vertex,
) -> bool {
	false
}
