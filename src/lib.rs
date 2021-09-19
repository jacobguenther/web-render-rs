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
pub mod input;
pub mod lights;
pub mod model;
pub mod program;
pub mod resources;
pub mod scene_graph;
pub mod shader;
pub mod warning;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{
	prelude::*,
	JsCast,
};
use web_sys::{
	RequestMode,
	WebGl2RenderingContext,
};

use cgmath::{
	conv::*,
	prelude::*,
	Matrix4,
};

use input::{
	mouse::MouseEventSubscriber,
	InputHandler,
};

use program::Program;
use resources::Resources;

use camera::Camera;
use context::Context;

use crate::input::mouse::MouseLocationDisplay;
use crate::input::slider::{
	CustomSuperellipse,
	CustomSupershape2D,
};
use crate::model::Drawable;
use crate::{
	config::{
		engine_config::EngineConfig,
		scene_config::SceneConfig,
		Config,
	},
	resources::fetch_json,
};

#[macro_export]
macro_rules! log {
	( $( $t:tt )* ) => {
		web_sys::console::log_1(&format!( $( $t )* ).into());
	}
}

#[wasm_bindgen]
pub fn init_panic_hook() {
	console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

#[wasm_bindgen]
pub async fn start(config: JsValue) -> Result<(), JsValue> {
	init_panic_hook();

	let config: Config = serde_wasm_bindgen::from_value(config)
		.map_err(|err| JsValue::from_str(&err.to_string()))?;

	let engine_config_json: JsValue = fetch_json(
		&format!("assets/{}", &config.engine_config_uri),
		RequestMode::SameOrigin,
	)
	.await?;
	let scene_config_json: JsValue = fetch_json(
		&format!("assets/{}", &config.scene_config_uri),
		RequestMode::SameOrigin,
	)
	.await?;

	let engine_config: EngineConfig =
		serde_wasm_bindgen::from_value(engine_config_json)
			.map_err(|err| JsValue::from_str(&err.to_string()))?;
	let scene_config: SceneConfig =
		serde_wasm_bindgen::from_value(scene_config_json)
			.map_err(|err| JsValue::from_str(&err.to_string()))?;

	let context = Rc::new(Context::new(&engine_config)?);

	let input_handler = InputHandler::new(&context.canvas);
	let mouse_display: Rc<RefCell<dyn MouseEventSubscriber>> =
		Rc::new(RefCell::new(MouseLocationDisplay::new(
			&context.document,
			"mouse-location",
		)?));
	input_handler
		.borrow_mut()
		.subscribe_for_mouse_move_event(&mouse_display);

	let gl = Rc::new(context.gl.clone());
	gl.clear_color(0.2, 0.2, 0.2, 1.0);
	gl.front_face(WebGl2RenderingContext::CCW); // default CW
	gl.cull_face(WebGl2RenderingContext::BACK); // default BACK
	gl.depth_func(WebGl2RenderingContext::LESS); // default LESS
	gl.enable(WebGl2RenderingContext::DEPTH_TEST);

	let mut resources = Resources::new(Rc::clone(&gl));
	resources.load_scene(&scene_config).await?;
	let camera = resources.cameras.get("main").unwrap().clone();
	let pbr_shader = resources.programs.get("pbr").unwrap().clone();

	setup_program(&gl, &pbr_shader, &camera);

	// let superellipse = Rc::new(RefCell::new(CustomSuperellipse::new(
	// 	&context.document,
	// 	&context.gl,
	// )));
	let supershape = Rc::new(RefCell::new(CustomSupershape2D::new(
		&context.document,
		&context.gl,
	)));

	let f = Rc::new(RefCell::new(None));
	let g = f.clone();

	let mut previous_time = 0.0;
	let mut frames = 0;
	let fps_span = context
		.document
		.get_element_by_id("fps")
		.ok_or("Expected element with id 'fps'")?;

	*g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
		let current_time = context.now().unwrap();
		frames += 1;
		if current_time > (previous_time + 1000.0) {
			let fps = (frames * 1000) as f64 / (current_time - previous_time);
			previous_time = current_time;
			frames = 0;
			fps_span.set_inner_html(&fps.to_string());
		}

		input_handler.borrow_mut().notify_subscribers();
		input_handler.borrow_mut().flush_events();

		gl.clear(
			WebGl2RenderingContext::COLOR_BUFFER_BIT
				| WebGl2RenderingContext::DEPTH_BUFFER_BIT,
		);

		{
			// let mut s = superellipse.borrow_mut();
			// s.update(&gl);
			// s.mesh.borrow().draw(&gl, &pbr_shader);

			let mut s = supershape.borrow_mut();
			s.update(&gl);
			s.mesh.borrow().draw(&gl, &pbr_shader);
		}

		request_animation_frame(f.borrow().as_ref().unwrap());
	}) as Box<dyn FnMut()>));

	request_animation_frame(g.borrow().as_ref().unwrap());

	Ok(())
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

fn setup_program(
	gl: &WebGl2RenderingContext,
	program: &Program,
	camera: &Camera,
) {
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
		gl.uniform_matrix4fv_with_f32_array(
			model_loc,
			false,
			&mat_4_to_array(&model_matrix),
		);
	}
}
#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(true, true);
	}
}
