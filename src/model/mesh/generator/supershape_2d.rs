use core::f64::consts::PI;

use cgmath::Point3;

use web_sys::WebGl2RenderingContext;

use crate::model::mesh::{
	generator::{
		vertex::Vertex,
		MeshGenerator,
		MeshGeneratorT,
		MeshMode,
	},
	Mesh,
};

pub struct Supershape2D {
	pub a: f32,
	pub b: f32,
	pub n_1: f32,
	pub n_2: f32,
	pub n_3: f32,
	pub m: f32,
}
impl Default for Supershape2D {
	fn default() -> Self {
		Self {
			a: 1.0,
			b: 1.0,
			n_1: 1.0,
			n_2: 1.0,
			n_3: 1.0,
			m: 1.0,
		}
	}
}
impl Supershape2D {
	pub fn new(a: f32, b: f32, n_1: f32, n_2: f32, n_3: f32, m: f32) -> Self {
		Self {
			a,
			b,
			n_1,
			n_2,
			n_3,
			m,
		}
	}
	pub fn points(&self) -> Vec<Point3<f32>> {
		let pi = PI as f32;
		let two_pi = 2.0 * pi;

		let steps = 512;
		let step_size_rad = two_pi / steps as f32;

		let a_inv = 1.0 / self.a;
		let b_inv = 1.0 / self.b;
		let m_div_4 = self.m / 4.0;
		let n_1_inv = 1.0 / self.n_1;

		let mut points = Vec::with_capacity(steps + 1);
		points.push(Point3::new(0.0, 0.0, 0.0));

		let mut theta = 0.0;
		for _ in 0..(steps + 1) {
			theta += step_size_rad;

			let part_1 = (a_inv * (theta * m_div_4).cos()).abs().powf(self.n_2);
			let part_2 = (b_inv * (theta * m_div_4).sin()).abs().powf(self.n_3);

			let mut r = (part_1 + part_2).powf(n_1_inv);
			if r != 0.0 {
				r = 1.0 / r;
			}

			let x = r * theta.cos();
			let y = r * theta.sin();
			let z = 0.0;

			let point = Point3::new(x, y, z);
			points.push(point);
		}

		points
	}
}
impl MeshGeneratorT for Supershape2D {
	fn generate_mesh(
		&mut self,
		gl: &WebGl2RenderingContext,
		mode: MeshMode,
	) -> Mesh {
		assert!(mode == MeshMode::Points || mode == MeshMode::TriangleFan);
		let points = self.points();
		let mut mesh_generator = MeshGenerator::new(None, None, None);

		for point in points {
			let vertex = Vertex::new(point);
			mesh_generator.push_vertex(&vertex);
		}

		mesh_generator.generate_mesh(gl, mode)
	}
}
