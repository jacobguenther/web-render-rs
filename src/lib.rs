// File: src/lib.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

pub mod camera;
pub mod config;
pub mod context;
pub mod gltf;
pub mod light;
pub mod model;
pub mod program;
pub mod resources;
pub mod shader;
pub mod warning;

use std::cell::RefCell;
use std::rc::Rc;

use program::Program;
use resources::Resources;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use cgmath::conv::*;
use cgmath::prelude::*;
use cgmath::Matrix4;
use cgmath::Vector3;

use camera::Camera;
use context::Context;
// use model::texture::Texture;
use model::{terrain::Terrain, Drawable};

#[macro_export]
macro_rules! log {
	( $( $t:tt )* ) => {
		web_sys::console::log_1(&format!( $( $t )* ).into());
	}
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

fn window() -> web_sys::Window {
	web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
	window()
		.request_animation_frame(f.as_ref().unchecked_ref())
		.expect("should register `requestAnimationFrame` OK");
}

pub fn mat_4_to_array(mat: &Matrix4<f32>) -> [f32; 16] {
	unsafe { std::mem::transmute::<[[f32; 4]; 4], [f32; 16]>(array4x4(*mat)) }
}

#[wasm_bindgen]
pub async fn start(config: JsValue) -> Result<(), JsValue> {
	let config: config::Config = match serde_wasm_bindgen::from_value(config) {
		Ok(c) => c,
		Err(e) => return Err(JsValue::from_str(&format!("{:?}", e))),
	};

	let context = Context::new(&config.canvas_id)?;
	context.set_size(config.width, config.height);

	let gl = Rc::new(context.gl);
	gl.clear_color(0.2, 0.2, 0.2, 1.0);
	gl.front_face(WebGl2RenderingContext::CCW); // default CW
	gl.cull_face(WebGl2RenderingContext::BACK); // default BACK
	gl.depth_func(WebGl2RenderingContext::LESS); // default LESS
	gl.enable(WebGl2RenderingContext::DEPTH_TEST);
	// gl.enable(WebGl2RenderingContext::SAMPLE_COVERAGE);
	// gl.sample_coverage(0.5, false);

	let chunk_size = 4;
	let terrain_scale = Vector3::new(1.0, 1.0, 1.0);
	let heights: &[&[f32]] = &[
		&[1.0, 1.0, 0.75, 0.0, 0.0],
		&[1.0, 1.0, 0.75, 0.0, 0.0],
		&[0.0, 1.0, 0.25, 0.0, 0.5],
		&[0.0, 1.0, 0.5, 0.0, 0.5],
		&[0.0, 1.0, 0.25, 0.0, 0.5],
	];
	let terrain = Terrain::generate(&gl, chunk_size, &terrain_scale, heights)?;

	let mut resources = Resources::new(gl.as_ref());
	let models = resources.load_models(&config.models)?;
	let _shaders = resources.load_shaders(&config.shaders).await?;
	let (_programs, program_warnings) = resources.load_programs(&config.programs)?;
	log!("{:?}", program_warnings);
	let pbr_shader = Rc::clone(resources.programs.get("pbr").unwrap());
	let terrain_shader = Rc::clone(resources.programs.get("terrain").unwrap());
	let camera = Camera::new(&config.camera, config.width, config.height);

	setup_program(&gl, &pbr_shader, &camera);
	setup_program(&gl, &terrain_shader, &camera);

	let f = Rc::new(RefCell::new(None));
	let g = f.clone();

	let mut angle = 0.0;
	*g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
		gl.clear(
			WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
		);

		if true {
			setup_program(&gl, &pbr_shader, &camera);
			let model_matrix = Matrix4::from_angle_y(cgmath::Deg(angle));
			angle += 0.5;
			let model_loc = pbr_shader.uniform_locations.get("MODEL_MATRIX");
			gl.uniform_matrix4fv_with_f32_array(model_loc, false, &mat_4_to_array(&model_matrix));
			for model in models.iter() {
				model.draw(&gl, &pbr_shader);
			}
		}
		if true {
			setup_program(&gl, &terrain_shader, &camera);
			let t = (chunk_size / 2) as f32;
			let translation = Vector3::<f32>::new(-t, 0.0, -t);
			let model_matrix = Matrix4::from_translation(translation);
			let model_loc = terrain_shader.uniform_locations.get("MODEL_MATRIX");
			gl.uniform_matrix4fv_with_f32_array(model_loc, false, &mat_4_to_array(&model_matrix));
			terrain.draw(&gl, &terrain_shader);
		}

		gl.flush();
		request_animation_frame(f.borrow().as_ref().unwrap());
	}) as Box<dyn FnMut()>));

	request_animation_frame(g.borrow().as_ref().unwrap());

	Ok(())
}

fn setup_program(gl: &WebGl2RenderingContext, program: &Program, camera: &Camera) {
	gl.use_program(Some(&program.program));
	let uniform_locations = &program.uniform_locations;
	{
		let camera_pos_loc = uniform_locations.get("CAMERA_POS");
		let camera_pos = camera.eye;
		gl.uniform3f(camera_pos_loc, camera_pos.x, camera_pos.y, camera_pos.z);
	}

	let model_matrix = Matrix4::identity();
	{
		let projection_loc = uniform_locations.get("PROJECTION_MATRIX");
		let view_loc = uniform_locations.get("VIEW_MATRIX");
		let model_loc = uniform_locations.get("MODEL_MATRIX");

		gl.uniform_matrix4fv_with_f32_array(
			projection_loc,
			false,
			&mat_4_to_array(&camera.perspective_matrix()),
		);
		gl.uniform_matrix4fv_with_f32_array(
			view_loc,
			false,
			&mat_4_to_array(&camera.view_matrix()),
		);
		gl.uniform_matrix4fv_with_f32_array(model_loc, false, &mat_4_to_array(&model_matrix));
	}
}
#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(true, true);
	}
}
