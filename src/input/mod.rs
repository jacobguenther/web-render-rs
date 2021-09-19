// File: src/input.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

pub mod mouse;
pub mod slider;

use macros::*;
use mouse::{
	MouseEvent,
	MouseEventSubscriber,
};
use std::{
	cell::RefCell,
	rc::Rc,
};
use wasm_bindgen::{
	prelude::*,
	JsCast,
};
use web_sys::HtmlCanvasElement;

type MouseSubscriberWrapper = Rc<RefCell<dyn MouseEventSubscriber>>;
// type SliderEventWrapper = Rc<RefCell<dyn SliderEventSubscriber>>;

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
	// slider_events: HashMap<String, SliderEvent>,
	// slider_event_subscribers: HashMap<String, SliderEventWrapper>,
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
			// slider_events: HashMap::new(),
			// slider_event_subscribers: HashMap::new(),
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
			let input_handler = Rc::clone(&input_handler);
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
			let input_handler = Rc::clone(&input_handler);
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
