// File: src/input.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use macros::*;

use std::{
	cell::RefCell,
	rc::Rc,
};
use wasm_bindgen::{
	prelude::*,
	JsCast,
};
use web_sys::HtmlCanvasElement;

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

type MouseSubscriberWrapper = Rc<RefCell<dyn MouseEventSubscriber>>;

#[derive(Clone)]
pub struct InputHandler {
	mouse_down_events: Vec<MouseEvent>,
	mouse_up_events: Vec<MouseEvent>,
	mouse_move_events: Vec<MouseEvent>,
	mouse_enter_event: Option<MouseEvent>,
	mouse_leave_event: Option<MouseEvent>,
	mouse_down_event_subscribers: Vec<MouseSubscriberWrapper>,
	mouse_up_event_subscribers: Vec<MouseSubscriberWrapper>,
	mouse_move_event_subscribers: Vec<MouseSubscriberWrapper>,
	mouse_enter_event_subscribers: Vec<MouseSubscriberWrapper>,
	mouse_leave_event_subscribers: Vec<MouseSubscriberWrapper>,
}

impl InputHandler {
	pub fn new(canvas: &HtmlCanvasElement) -> Rc<RefCell<Self>> {
		let input_handler = Rc::new(RefCell::new(Self {
			mouse_down_events: Vec::new(),
			mouse_up_events: Vec::new(),
			mouse_move_events: Vec::new(),
			mouse_enter_event: None,
			mouse_leave_event: None,
			mouse_down_event_subscribers: Vec::new(),
			mouse_up_event_subscribers: Vec::new(),
			mouse_move_event_subscribers: Vec::new(),
			mouse_enter_event_subscribers: Vec::new(),
			mouse_leave_event_subscribers: Vec::new(),
		}));
		create_callback!(
			"mouse_down_events web_sys::MouseEvent MouseEvent canvas mousedown"
		);
		create_callback!(
			"mouse_up_events web_sys::MouseEvent MouseEvent canvas mouseup"
		);
		create_callback!(
			"mouse_move_events web_sys::MouseEvent MouseEvent canvas mousemove"
		);
		{
			let input_handler = input_handler.clone();
			let closure =
				Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
					input_handler.borrow_mut().mouse_enter_event =
						Some(MouseEvent::from(event));
				}) as Box<dyn FnMut(_)>);
			let _err = canvas.add_event_listener_with_callback(
				"mouseenter",
				closure.as_ref().unchecked_ref(),
			);
			closure.forget();
		}
		{
			let input_handler = input_handler.clone();
			let closure =
				Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
					input_handler.borrow_mut().mouse_leave_event =
						Some(MouseEvent::from(event));
				}) as Box<dyn FnMut(_)>);
			let _err = canvas.add_event_listener_with_callback(
				"mouseleave",
				closure.as_ref().unchecked_ref(),
			);
			closure.forget();
		}

		input_handler
	}
	pub fn notify_subscribers(&self) {
		self.notify_mouse_down_subscribers();
		self.notify_mouse_up_subscribers();
		self.notify_mouse_move_subscribers();
		self.notify_mouse_enter_subscribers();
		self.notify_mouse_leave_subscribers();
	}
	pub fn flush_events(&mut self) {
		self.mouse_down_events.clear();
		self.mouse_up_events.clear();
		self.mouse_move_events.clear();
		self.mouse_enter_event = None;
		self.mouse_leave_event = None;
	}
	pub fn flush_subscribers(&mut self) {
		self.mouse_down_event_subscribers.clear();
		self.mouse_up_event_subscribers.clear();
		self.mouse_move_event_subscribers.clear();
		self.mouse_enter_event_subscribers.clear();
		self.mouse_leave_event_subscribers.clear();
	}

	pub fn subscribe_for_mouse_down_event(
		&mut self,
		subscriber: &Rc<RefCell<dyn MouseEventSubscriber>>,
	) {
		self.mouse_down_event_subscribers.push(subscriber.clone());
	}
	pub fn subscribe_for_mouse_up_event(
		&mut self,
		subscriber: &Rc<RefCell<dyn MouseEventSubscriber>>,
	) {
		self.mouse_up_event_subscribers.push(subscriber.clone());
	}
	pub fn subscribe_for_mouse_move_event(
		&mut self,
		subscriber: &Rc<RefCell<dyn MouseEventSubscriber>>,
	) {
		self.mouse_move_event_subscribers.push(subscriber.clone());
	}
	pub fn subscribe_for_mouse_enter_event(
		&mut self,
		subscriber: &Rc<RefCell<dyn MouseEventSubscriber>>,
	) {
		self.mouse_enter_event_subscribers.push(subscriber.clone());
	}
	pub fn subscribe_for_mouse_leave_event(
		&mut self,
		subscriber: &Rc<RefCell<dyn MouseEventSubscriber>>,
	) {
		self.mouse_leave_event_subscribers.push(subscriber.clone());
	}

	fn notify_mouse_down_subscribers(&self) {
		gen_notify_event_body_queue!(
			"mouse_down_event_subscribers mouse_down_events notify_down"
		);
	}
	fn notify_mouse_up_subscribers(&self) {
		gen_notify_event_body_queue!(
			"mouse_up_event_subscribers mouse_up_events notify_up"
		);
	}
	fn notify_mouse_move_subscribers(&self) {
		gen_notify_event_body_queue!(
			"mouse_move_event_subscribers mouse_move_events notify_move"
		);
	}
	fn notify_mouse_enter_subscribers(&self) {
		gen_notify_event_body_single!(
			"mouse_enter_event_subscribers mouse_enter_event notify_enter"
		);
	}
	fn notify_mouse_leave_subscribers(&self) {
		gen_notify_event_body_single!(
			"mouse_leave_event_subscribers mouse_leave_event notify_leave"
		);
	}
}
