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

use input::{
	InputHandler,
	MouseEvent,
	MouseEventSubscriber,
};
use program::Program;
use rctree::Node;
use resources::Resources;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
	CanvasRenderingContext2d,
	Document,
	Element,
	HtmlCanvasElement,
	WebGl2RenderingContext,
};

use cgmath::conv::*;
use cgmath::prelude::*;
use cgmath::Matrix4;
use cgmath::Vector3;

use camera::Camera;
use context::Context;
use model::Drawable;

use crate::{
	model::{
		material::Material,
		mesh::generator::{
			// icosphere::generate_icosphere,
			terrain::Terrain,
			uv_sphere::generate_uv_sphere,
		},
		texture::Sampler,
	},
	resources::traits::NewResourceT,
	scene_graph::{
		draw_node,
		set_local_matrix,
		NodeData,
		NodeTypeData,
		Transform,
	},
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

struct MouseLocationDisplay {
	element: Element,
}
impl MouseLocationDisplay {
	pub fn new(document: &Document, id: &str) -> Result<Self, &'static str> {
		let element = document
			.get_element_by_id(id)
			.ok_or("Failed to get element for MouseLocationDisplay")?;
		Ok(Self { element })
	}
	fn on_event(&self, event: &MouseEvent) {
		let location = format!("x: {}, y: {}", event.x, event.y);
		self.element.set_inner_html(&location);
	}
}
impl MouseEventSubscriber for MouseLocationDisplay {
	fn notify_down(&mut self, event: &MouseEvent) {
		self.on_event(event);
	}
	fn notify_up(&mut self, event: &MouseEvent) {
		self.on_event(event);
	}
	fn notify_move(&mut self, event: &MouseEvent) {
		self.on_event(event);
	}
	fn notify_enter(&mut self, event: &MouseEvent) {
		self.on_event(event);
	}
	fn notify_leave(&mut self, event: &MouseEvent) {
		self.on_event(event);
	}
}

#[wasm_bindgen]
pub async fn start(config: JsValue) -> Result<(), JsValue> {
	init_panic_hook();

	let config: config::Config = match serde_wasm_bindgen::from_value(config) {
		Ok(c) => c,
		Err(e) => return Err(JsValue::from_str(&format!("{:?}", e))),
	};

	let context = Rc::new(Context::new(&config.canvas_id)?);
	context.set_size(config.width, config.height);

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
	// gl.enable(WebGl2RenderingContext::SAMPLE_COVERAGE);
	// gl.sample_coverage(0.5, false);

	let mut resources = Resources::new(Rc::clone(&gl));
	let _models = resources.load_models(&config.models)?;
	let _shaders = resources.load_shaders(&config.shaders).await?;
	let (_programs, program_warnings) =
		resources.load_programs(&config.programs)?;
	log!("{:?}", program_warnings);
	let pbr_shader = Rc::clone(resources.programs.get("pbr").unwrap());
	let terrain_shader = Rc::clone(resources.programs.get("terrain").unwrap());
	let camera = Camera::new(&config.camera, config.width, config.height);

	let t0 = context.now()?;

	let override_size = 101;
	let (heights, _chunk_size) = {
		let image = resources::fetch_image(
			&context.document,
			"terrain_height_map",
			"assets/height_maps/test.png",
			"image_wrapper",
		)
		.await?;
		let width = image.width() as usize;
		let height = image.height() as usize;
		if width != height {
			return Err(JsValue::from_str(
				"Height map must have equal dimensions",
			));
		}

		let canvas = context
			.document
			.create_element("canvas")?
			.dyn_into::<HtmlCanvasElement>()?;
		canvas.set_width(width as u32);
		canvas.set_height(height as u32);

		let context_2d = canvas
			.get_context("2d")?
			.ok_or_else(|| JsValue::from_str("Failed to context object"))?
			.dyn_into::<CanvasRenderingContext2d>()?;
		let dx = 0.0;
		let dy = 0.0;
		let dw = width as f64;
		let dh = height as f64;
		context_2d.draw_image_with_html_image_element_and_dw_and_dh(
			&image, dx, dy, dw, dh,
		)?;

		let canvas_wrapper = context
			.document
			.get_element_by_id("canvas_wrapper")
			.ok_or_else(|| {
				JsValue::from_str(
					"Failed to get element with id canvas_wrapper",
				)
			})?;
		canvas_wrapper.append_child(&canvas)?;

		let sx = dx;
		let sy = dy;
		let sw = dw;
		let sh = dh;
		let image_data = context_2d.get_image_data(sx, sy, sw, sh)?;
		let data = image_data.data();

		let mut heights = Vec::with_capacity(width * height);
		// let mut i = 0;
		// for _row in 0..width {
		// 	for _col in 0..height {
		for row in 0..override_size {
			let mut i = row * width * 4;
			for _col in 0..override_size {
				let height = data[i] as f32;
				heights.push(height);
				i += 4;
			}
		}
		(heights, width as usize - 1)
	};
	let chunk_size = override_size - 1;

	let t1 = context.now()?;
	crate::log!("load height map: {}ms", t1 - t0);

	let t0 = context.now()?;

	let terrain_scale = Vector3::new(1.0, 0.3, 1.0);
	let terrain = Terrain::generate(&gl, chunk_size, &terrain_scale, &heights)?;

	let t1 = context.now()?;
	crate::log!("generate terrain: {}ms", t1 - t0);

	setup_program(&gl, &pbr_shader, &camera);
	setup_program(&gl, &terrain_shader, &camera);

	let texture_path =
		// "assets/textures/earthmap1k.jpg";
		"assets/textures/mars_viking_colorized.jpg";
	let earth_image = resources::fetch_image(
		&context.document,
		"earth_albedo",
		texture_path,
		"image_wrapper",
	)
	.await?;

	let earth_texture_sampler = Rc::new(Sampler::default());
	let earth_texture = resources.new_texture(
		"earth_albedo",
		&earth_image,
		0,
		&earth_texture_sampler,
	)?;
	let mut earth_material = Material::default();
	earth_material.diffuse_tex = Some(earth_texture.clone());

	let mut solar_system = Node::<NodeData>::new(NodeData::default());

	let sphere_0 = generate_uv_sphere(&gl, 1.0, 16, 16);
	let mut sun = Node::<NodeData>::new(NodeData::new(
		Transform::default(),
		NodeTypeData::Mesh(Rc::clone(&sphere_0)),
	));

	let mut earth_orbit = Node::<NodeData>::new(NodeData::new(
		Transform::new(&Matrix4::from_translation(Vector3::new(6.0, 0.0, 0.0))),
		NodeTypeData::Transform,
	));
	let sphere_1 = generate_uv_sphere(&gl, 2.0, 16, 16);
	let mut sphere_1 = sphere_1.as_ref().clone();
	sphere_1.material = Rc::new(earth_material);
	let mut earth = Node::<NodeData>::new(NodeData::new(
		Transform::new(&Matrix4::one()),
		NodeTypeData::Mesh(Rc::new(sphere_1)),
	));

	let mut moon_orbit = Node::<NodeData>::new(NodeData::new(
		Transform::new(&Matrix4::from_translation(Vector3::new(3.0, 0.0, 0.0))),
		NodeTypeData::Transform,
	));
	let sphere_2 = generate_uv_sphere(&gl, 0.3, 6, 6);
	let mut moon = Node::<NodeData>::new(NodeData::new(
		Transform::new(&Matrix4::one()),
		NodeTypeData::Mesh(Rc::clone(&sphere_2)),
	));

	solar_system.append(sun.clone());
	earth_orbit.append(earth.clone());
	moon_orbit.append(moon.clone());
	earth_orbit.append(moon_orbit.clone());
	solar_system.append(earth_orbit.clone());
	set_local_matrix(&mut solar_system, &Matrix4::one());

	// F = GMm / r^2 = mu * m / r^2
	// A = mu / r^2

	let f = Rc::new(RefCell::new(None));
	let g = f.clone();

	let mut previous_time = 0.0;
	let mut frames = 0;
	let fps_span = context
		.document
		.get_element_by_id("fps")
		.ok_or("Expected element with id 'fps'")?;

	let mut t; //  = Vector3::new(0.0, 0.0, 0.0);
	t = (chunk_size / 2) as f32 * terrain_scale;
	t.y = 0.0;
	t = -t;
	let t = t;

	let mut angle = 0.0;

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

		if false {
			setup_program(&gl, &pbr_shader, &camera);
			let model_matrix = Matrix4::from_angle_y(cgmath::Deg(angle));
			let model_loc = pbr_shader.uniform_locations.get("MODEL_MATRIX");
			gl.uniform_matrix4fv_with_f32_array(
				model_loc,
				false,
				&mat_4_to_array(&model_matrix),
			);
			for (_id, model) in resources.models.iter() {
				model.draw(&gl, &pbr_shader);
			}
		}

		if false {
			setup_program(&gl, &terrain_shader, &camera);
			let translation = Matrix4::from_translation(t);
			let rotation = Matrix4::from_angle_y(cgmath::Deg(angle));
			// let scale = Matrix4::from_nonuniform_scale(
			// 	terrain_scale.x,
			// 	terrain_scale.y,
			// 	terrain_scale.z,
			// );

			// rotation * translation * scale;
			let model_matrix = rotation * translation;
			let model_loc =
				terrain_shader.uniform_locations.get("MODEL_MATRIX");
			gl.uniform_matrix4fv_with_f32_array(
				model_loc,
				false,
				&mat_4_to_array(&model_matrix),
			);
			terrain.draw(&gl, &terrain_shader);
		}

		if true {
			setup_program(&gl, &pbr_shader, &camera);
			let earth_orbit_angle = 0.3;
			{
				let transform = *earth_orbit.borrow_mut().ref_transform();
				let local_matrix =
					Matrix4::from_angle_y(cgmath::Deg(earth_orbit_angle))
						* transform.local_matrix();
				set_local_matrix(&mut earth_orbit, &local_matrix);
			}
			let earth_angle = 1.0;
			{
				let transform = *earth.borrow_mut().ref_transform();
				let local_matrix =
					Matrix4::from_angle_y(cgmath::Deg(earth_angle))
						* transform.local_matrix();
				set_local_matrix(&mut earth, &local_matrix);
			}
			let moon_orbit_angle = -0.1;
			{
				let transform = *moon_orbit.borrow_mut().ref_transform();
				let local_matrix =
					Matrix4::from_angle_y(cgmath::Deg(moon_orbit_angle))
						* transform.local_matrix();
				set_local_matrix(&mut moon_orbit, &local_matrix);
			}
			let moon_angle = -0.1;
			{
				let transform = *moon.borrow_mut().ref_transform();
				let local_matrix =
					Matrix4::from_angle_y(cgmath::Deg(moon_angle))
						* transform.local_matrix();
				set_local_matrix(&mut moon, &local_matrix);
			}
			let sun_angle = 0.1;
			{
				let transform = *sun.borrow_mut().ref_transform();
				let local_matrix =
					Matrix4::from_angle_y(cgmath::Deg(sun_angle))
						* transform.local_matrix();
				set_local_matrix(&mut sun, &local_matrix);
			}

			draw_node(&solar_system, &gl, &pbr_shader);
		}

		if false {
			angle += 0.5;
		}

		request_animation_frame(f.borrow().as_ref().unwrap());
	}) as Box<dyn FnMut()>));

	request_animation_frame(g.borrow().as_ref().unwrap());

	Ok(())
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
