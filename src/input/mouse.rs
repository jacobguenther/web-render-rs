// File: src/input/slider.rs
// Author: Jacob Guenther
// Date created: Sept 2021
// License: AGPLv3
//
// Description:

use web_sys::{
	Document,
	Element,
};

#[derive(Copy, Clone, Debug)]
pub struct MouseEvent {
	pub x: i32,
	pub y: i32,

	pub left_btn: bool,
	pub right_btn: bool,
	pub middle_btn: bool,
	pub aux_btn: bool,
	pub back_btn: bool,
	pub forward_btn: bool,

	pub ctrl: bool,
	pub shift: bool,
	pub alt: bool,
	pub meta: bool,
}
impl Default for MouseEvent {
	fn default() -> Self {
		Self {
			x: 0,
			y: 0,
			left_btn: false,
			right_btn: false,
			middle_btn: false,
			aux_btn: false,
			back_btn: false,
			forward_btn: false,
			ctrl: false,
			shift: false,
			alt: false,
			meta: false,
		}
	}
}
impl From<web_sys::MouseEvent> for MouseEvent {
	fn from(other: web_sys::MouseEvent) -> Self {
		let buttons = other.buttons();
		Self {
			x: other.offset_x(),
			y: other.offset_y(),
			left_btn: buttons & (1 << 0) != 0,
			right_btn: buttons & (1 << 1) != 0,
			middle_btn: buttons & (1 << 2) != 0,
			aux_btn: buttons & (1 << 3) != 0,
			back_btn: buttons & (1 << 4) != 0,
			forward_btn: buttons & (1 << 5) != 0,
			ctrl: other.ctrl_key(),
			shift: other.shift_key(),
			alt: other.alt_key(),
			meta: other.meta_key(),
		}
	}
}
pub trait MouseEventSubscriber {
	fn notify_down(&mut self, _event: &MouseEvent) {}
	fn notify_up(&mut self, _event: &MouseEvent) {}
	fn notify_move(&mut self, _event: &MouseEvent) {}
	fn notify_enter(&mut self, _event: &MouseEvent) {}
	fn notify_leave(&mut self, _event: &MouseEvent) {}
}

pub struct MouseLocationDisplay {
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
