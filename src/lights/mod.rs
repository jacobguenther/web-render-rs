// File: src/lights/mod.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use cgmath::{
	Point3,
	Vector3,
};

#[derive(Copy, Clone, Debug)]
pub struct DirectionLight {
	pub dir: Vector3<f32>,
	pub color: Vector3<f32>,
}

#[derive(Copy, Clone, Debug)]
pub struct PointLight {
	pub position: Point3<f32>,
	pub color: Vector3<f32>,
}
