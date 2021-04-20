// File: src/context.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use cgmath::Point3;

#[derive(Copy, Clone, Debug)]
pub struct PointLight {
	pub position: Point3<f32>,
	pub color: Point3<f32>,
}
