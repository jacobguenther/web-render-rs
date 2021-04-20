// File: src/context.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, WebGl2RenderingContext, Window};

pub struct Context {
	pub window: Window,
	pub document: Document,
	pub canvas: HtmlCanvasElement,
	pub gl: WebGl2RenderingContext,
}
impl Context {
	pub fn new(canvas_id: &str) -> Result<Self, &'static str> {
		let window = web_sys::window().ok_or("Failed to get window")?;
		let document = window.document().ok_or("Failed to get document")?;
		let canvas = document
			.get_element_by_id(canvas_id)
			.ok_or("Faild to find element with given id for canvas")?
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.map_err(|_err| "Element is not a canvas")?;
		let gl: WebGl2RenderingContext = canvas
			.get_context("webgl2")
			.map_err(|_err| "Failed to create webgl context")?
			.ok_or("Failed to create webgl context")?
			.dyn_into::<WebGl2RenderingContext>()
			.map_err(|_err| "Failed to create webgl context")?;
		Ok(Self {
			window,
			document,
			canvas,
			gl,
		})
	}
	pub fn set_size(&self, width: u32, height: u32) {
		self.canvas.set_width(width);
		self.canvas.set_height(height);
		let width = self.canvas.client_width();
		let height = self.canvas.client_height();
		self.canvas.set_width(width as u32);
		self.canvas.set_height(height as u32);
		self.gl.viewport(0, 0, width, height);
	}
}
