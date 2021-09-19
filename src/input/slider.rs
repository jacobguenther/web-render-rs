// File: src/input/slider.rs
// Author: Jacob Guenther
// Date created: Sept 2021
// License: AGPLv3
//
// Description:

use std::{
	cell::RefCell,
	rc::Rc,
};
use wasm_bindgen::{
	prelude::*,
	JsCast,
};
use web_sys::{
	Document,
	Element,
	WebGl2RenderingContext,
};

use crate::model::{
	mesh::{
		generator::{
			superellipse::Superellipse,
			supershape_2d::Supershape2D,
			MeshGeneratorT,
			MeshMode,
		},
		Mesh,
	},
	Drawable,
};

pub struct Slider {
	slider: Rc<web_sys::HtmlInputElement>,
	value_span: Element,
	value: f32,
}
impl Slider {
	pub fn new(
		document: &Document,
		slider_id: &str,
		span_id: &str,
	) -> Result<Rc<RefCell<Self>>, &'static str> {
		let value_span = document
			.get_element_by_id(span_id)
			.ok_or("Failed to get element for RangeValueDisplay")?;
		let slider = Rc::new(
			document
				.get_element_by_id(slider_id)
				.ok_or("Failed to get slider for RangeValueDisplay")?
				.dyn_into::<web_sys::HtmlInputElement>()
				.map_err(|_err| "Element is not a canvas")?,
		);

		let this = Rc::new(RefCell::new(Self {
			slider: Rc::clone(&slider),
			value_span,
			value: 0.0,
		}));
		this.borrow_mut().on_change();

		{
			let this = Rc::clone(&this);
			let closure =
				Closure::wrap(Box::new(move |_: web_sys::InputEvent| {
					this.borrow_mut().on_change();
				}) as Box<dyn FnMut(_)>);
			let _err = slider.add_event_listener_with_callback(
				"input",
				closure.as_ref().unchecked_ref(),
			);
			closure.forget();
		}

		Ok(this)
	}
	pub fn value(&self) -> f32 {
		self.value
	}
	fn on_change(&mut self) {
		let value = self.slider.value();
		self.value = str::parse::<f32>(&value).unwrap();
		self.value_span.set_inner_html(&value);
	}
}

pub struct CustomSuperellipse {
	a_slider: Rc<RefCell<Slider>>,
	b_slider: Rc<RefCell<Slider>>,
	n_slider: Rc<RefCell<Slider>>,

	superellipse: Superellipse,
	pub mesh: Rc<RefCell<Mesh>>,
}
impl Drawable for CustomSuperellipse {
	fn draw(
		&self,
		gl: &WebGl2RenderingContext,
		program: &crate::program::Program,
	) {
		self.mesh.borrow().draw(gl, program);
	}
}
impl CustomSuperellipse {
	pub fn new(document: &Document, gl: &WebGl2RenderingContext) -> Self {
		let a_slider = Slider::new(
			document,
			"superellipse-input-a",
			"superellipse-value-a",
		)
		.unwrap();
		let b_slider = Slider::new(
			document,
			"superellipse-input-b",
			"superellipse-value-b",
		)
		.unwrap();
		let n_slider = Slider::new(
			document,
			"superellipse-input-n",
			"superellipse-value-n",
		)
		.unwrap();

		let mut superellipse = Superellipse::new(
			a_slider.borrow().value(),
			b_slider.borrow().value(),
			n_slider.borrow().value(),
		);
		let mesh =
			superellipse.generate_rc_refcell_mesh(gl, MeshMode::TriangleFan);
		Self {
			a_slider,
			b_slider,
			n_slider,

			superellipse,
			mesh,
		}
	}
	pub fn update(&mut self, gl: &WebGl2RenderingContext) {
		let mut should_update = false;

		let a = self.a_slider.borrow().value();
		if self.superellipse.a != a {
			self.superellipse.a = a;
			should_update = true;
		}
		let b = self.b_slider.borrow().value();
		if self.superellipse.b != b {
			self.superellipse.b = b;
			should_update = true;
		}
		let n = self.n_slider.borrow().value();
		if self.superellipse.n != n {
			self.superellipse.n = n;
			should_update = true;
		}
		if should_update {
			self.mesh = self
				.superellipse
				.generate_rc_refcell_mesh(gl, MeshMode::TriangleFan);
		}
	}
}

pub struct CustomSupershape2D {
	a_slider: Rc<RefCell<Slider>>,
	b_slider: Rc<RefCell<Slider>>,
	n_1_slider: Rc<RefCell<Slider>>,
	n_2_slider: Rc<RefCell<Slider>>,
	n_3_slider: Rc<RefCell<Slider>>,
	m_slider: Rc<RefCell<Slider>>,

	supershape: Supershape2D,
	pub mesh: Rc<RefCell<Mesh>>,
}
impl Drawable for CustomSupershape2D {
	fn draw(
		&self,
		gl: &WebGl2RenderingContext,
		program: &crate::program::Program,
	) {
		self.mesh.borrow().draw(gl, program);
	}
}
impl CustomSupershape2D {
	pub fn new(document: &Document, gl: &WebGl2RenderingContext) -> Self {
		let a_slider = Slider::new(
			document,
			"supershape-2d-input-a",
			"supershape-2d-value-a",
		)
		.unwrap();
		let b_slider = Slider::new(
			document,
			"supershape-2d-input-b",
			"supershape-2d-value-b",
		)
		.unwrap();
		let n_1_slider = Slider::new(
			document,
			"supershape-2d-input-n-1",
			"supershape-2d-value-n-1",
		)
		.unwrap();
		let n_2_slider = Slider::new(
			document,
			"supershape-2d-input-n-2",
			"supershape-2d-value-n-2",
		)
		.unwrap();
		let n_3_slider = Slider::new(
			document,
			"supershape-2d-input-n-3",
			"supershape-2d-value-n-3",
		)
		.unwrap();
		let m_slider = Slider::new(
			document,
			"supershape-2d-input-m",
			"supershape-2d-value-m",
		)
		.unwrap();

		let mut supershape = Supershape2D::new(
			a_slider.borrow().value(),
			b_slider.borrow().value(),
			n_1_slider.borrow().value(),
			n_2_slider.borrow().value(),
			n_3_slider.borrow().value(),
			m_slider.borrow().value(),
		);
		let mesh =
			supershape.generate_rc_refcell_mesh(gl, MeshMode::TriangleFan);
		Self {
			a_slider,
			b_slider,
			n_1_slider,
			n_2_slider,
			n_3_slider,
			m_slider,

			supershape,
			mesh,
		}
	}
	pub fn update(&mut self, gl: &WebGl2RenderingContext) {
		let mut should_update = false;

		let a = self.a_slider.borrow().value();
		if self.supershape.a != a {
			self.supershape.a = a;
			should_update = true;
		}
		let b = self.b_slider.borrow().value();
		if self.supershape.b != b {
			self.supershape.b = b;
			should_update = true;
		}
		let n_1 = self.n_1_slider.borrow().value();
		if self.supershape.n_1 != n_1 {
			self.supershape.n_1 = n_1;
			should_update = true;
		}
		let n_2 = self.n_2_slider.borrow().value();
		if self.supershape.n_2 != n_2 {
			self.supershape.n_2 = n_2;
			should_update = true;
		}
		let n_3 = self.n_3_slider.borrow().value();
		if self.supershape.n_3 != n_3 {
			self.supershape.n_3 = n_3;
			should_update = true;
		}
		let m = self.m_slider.borrow().value();
		if self.supershape.m != m {
			self.supershape.m = m;
			should_update = true;
		}
		if should_update {
			self.mesh = self
				.supershape
				.generate_rc_refcell_mesh(gl, MeshMode::TriangleFan);
		}
	}
}
