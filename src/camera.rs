// File: src/camera.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use crate::config::CameraIntermediate;
use cgmath::{
	InnerSpace,
	Matrix3,
	Matrix4,
	Point3,
	Quaternion,
	Vector3,
};
// use cgmath::EuclideanSpace;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
	pub fovy: f32,
	pub aspect: f32,
	pub znear: f32,
	pub zfar: f32,

	pub eye: Point3<f32>,
	pub center: Point3<f32>,
	pub up: Vector3<f32>,

	pub orientation: Quaternion<f32>,
	pub position: Point3<f32>,
}
impl Camera {
	pub fn new(
		intermediate: &CameraIntermediate,
		width: u32,
		height: u32,
	) -> Self {
		let eye = Point3::from(intermediate.eye);
		let center = Point3::from(intermediate.center);

		let up = Vector3::from(intermediate.up); //.normalize();

		// fix this
		let front = (center - eye).normalize();
		let side = up.cross(front).normalize();
		let new_up = front.cross(side).normalize();
		let basis = Matrix3::<f32>::from_cols(front, side, new_up);

		Self {
			fovy: intermediate.fovy,
			aspect: width as f32 / height as f32,
			znear: intermediate.znear,
			zfar: intermediate.zfar,

			eye,
			center,
			up,

			orientation: Quaternion::from(basis),
			position: Point3::from(intermediate.eye),
		}
	}
	pub fn view_matrix(&self) -> Matrix4<f32> {
		// let rotation = Matrix4::from(self.orientation);
		// let pos: Vector3<f32> = self.position.to_vec();
		// let translation = Matrix4::from_translation(pos);
		// translation * rotation

		Matrix4::look_at_rh(self.eye, self.center, self.up)
	}
	pub fn perspective_matrix(&self) -> Matrix4<f32> {
		cgmath::perspective(
			cgmath::Deg(self.fovy),
			self.aspect,
			self.znear,
			self.zfar,
		)
	}
}
