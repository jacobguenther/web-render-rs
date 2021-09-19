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

pub struct Superellipse {
	pub a: f32,
	pub b: f32,
	pub n: f32,
}
impl Default for Superellipse {
	fn default() -> Self {
		Self {
			a: 1.0,
			b: 1.0,
			n: 2.0,
		}
	}
}
fn sgn(n: f32) -> f32 {
	match n {
		n if n.is_sign_positive() => 1.0,
		n if n.is_sign_negative() => -1.0,
		_ => 0.0,
	}
}
impl Superellipse {
	pub fn new(a: f32, b: f32, n: f32) -> Self {
		Self { a, b, n }
	}
	pub fn points(&self) -> Vec<Point3<f32>> {
		let mut points = Vec::new();
		let steps = 64;
		let steps_f32 = steps as f32;
		let step_size_rad = 1.0 / steps_f32;

		let pi = PI as f32;
		let two_pi = 2.0 * pi;
		let na = 2.0 / self.n;

		points.push(Point3::new(0.0, 0.0, 0.0));

		let first = Point3::new(self.a, 0.0, 0.0);
		points.push(first);
		for step in 1..steps {
			let ratio = step as f32 * step_size_rad;
			let theta = ratio * two_pi;

			let cos_theta = theta.cos();
			let sin_theta = theta.sin();

			let x = cos_theta.abs().powf(na) * self.a * sgn(cos_theta);
			let y = sin_theta.abs().powf(na) * self.b * sgn(sin_theta);
			let z = 0.0;

			let point = Point3::new(x, y, z);
			points.push(point);
		}
		points.push(first);
		points
	}
}
impl MeshGeneratorT for Superellipse {
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
