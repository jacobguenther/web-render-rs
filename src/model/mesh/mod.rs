// File: src/model/mesh.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

pub mod generator;

use std::{
	convert::TryFrom,
	rc::Rc,
};

use web_sys::{
	WebGl2RenderingContext,
	WebGlVertexArrayObject,
};

use super::buffer::Buffer;
use super::buffer_view::BufferView;
use super::material::Material;
use super::Drawable;
use crate::program::attribute_locations;
use crate::{
	config::scene_config::BufferViewConfig,
	program::Program,
};

#[derive(Copy, Clone, Debug)]
pub struct AttributeBufferViews {
	pub position: BufferView,
	pub normal: Option<BufferView>,
	pub tangent: Option<BufferView>,
	pub bitangent: Option<BufferView>,
	pub color: Option<BufferView>,
	pub texcoord_0: Option<BufferView>,
	pub texcoord_1: Option<BufferView>,
	pub texcoord_2: Option<BufferView>,
	pub texcoord_3: Option<BufferView>,
}
impl TryFrom<&[BufferViewConfig]> for AttributeBufferViews {
	type Error = &'static str;
	fn try_from(views: &[BufferViewConfig]) -> Result<Self, Self::Error> {
		let mut position = None;
		let mut normal = None;
		let mut tangent = None;
		let mut bitangent = None;
		let mut color = None;
		let mut texcoord_0 = None;
		let mut texcoord_1 = None;
		let mut texcoord_2 = None;
		let mut texcoord_3 = None;

		for view in views.iter() {
			match &view.id[..] {
				"POSITION" => position = Some(BufferView::new(view)),
				"NORMAL" => normal = Some(BufferView::new(view)),
				"TANGENT" => tangent = Some(BufferView::new(view)),
				"BITANGENT" => bitangent = Some(BufferView::new(view)),
				"COLOR" => color = Some(BufferView::new(view)),
				"TEXCOORD_0" => texcoord_0 = Some(BufferView::new(view)),
				"TEXCOORD_1" => texcoord_1 = Some(BufferView::new(view)),
				"TEXCOORD_2" => texcoord_2 = Some(BufferView::new(view)),
				"TEXCOORD_3" => texcoord_3 = Some(BufferView::new(view)),
				_ => (),
			}
		}

		let position =
			position.ok_or("Attribute views must contain a position view")?;
		Ok(AttributeBufferViews {
			position,
			normal,
			tangent,
			bitangent,
			color,
			texcoord_0,
			texcoord_1,
			texcoord_2,
			texcoord_3,
		})
	}
}

#[derive(Clone, Debug)]
pub struct Mesh {
	vao: WebGlVertexArrayObject,
	pub material: Rc<Material>,
	pub buffers: Vec<Rc<Buffer>>,
	pub index_view: Option<BufferView>,
	pub attribute_buffer_views: AttributeBufferViews,
	pub mode: u32,
}
impl Mesh {
	pub fn new(
		gl: &WebGl2RenderingContext,
		material: &Rc<Material>,
		buffers: &[Rc<Buffer>],
		index_view: &Option<BufferView>,
		attribute_buffer_views: &AttributeBufferViews,
		mode: u32,
	) -> Result<Self, &'static str> {
		let vao = Self::_create_vao(gl, attribute_buffer_views, buffers)?;
		Ok(Self {
			vao,
			material: material.clone(),
			buffers: buffers.to_owned(),
			index_view: *index_view,
			attribute_buffer_views: *attribute_buffer_views,
			mode,
		})
	}
	pub fn clean_up(&mut self, gl: &WebGl2RenderingContext) {
		gl.delete_vertex_array(Some(&self.vao));
		for buffer in self.buffers.iter() {
			gl.delete_buffer(Some(&buffer.handle));
		}
	}
	fn _create_vao(
		gl: &WebGl2RenderingContext,
		attribute_buffer_views: &AttributeBufferViews,
		buffers: &[Rc<Buffer>],
	) -> Result<WebGlVertexArrayObject, &'static str> {
		let vao = gl
			.create_vertex_array()
			.ok_or("Failed to create vertex array object")?;
		gl.bind_vertex_array(Some(&vao));

		Self::_bind_view(
			gl,
			attribute_locations::POSITION_LOCATION,
			&Some(attribute_buffer_views.position),
			buffers,
		);
		Self::_bind_view(
			gl,
			attribute_locations::NORMAL_LOCATION,
			&attribute_buffer_views.normal,
			buffers,
		);
		Self::_bind_view(
			gl,
			attribute_locations::TANGENT_LOCATION,
			&attribute_buffer_views.tangent,
			buffers,
		);
		Self::_bind_view(
			gl,
			attribute_locations::BITANGENT_LOCATION,
			&attribute_buffer_views.bitangent,
			buffers,
		);
		Self::_bind_view(
			gl,
			attribute_locations::COLOR_LOCATION,
			&attribute_buffer_views.color,
			buffers,
		);
		Self::_bind_view(
			gl,
			attribute_locations::TEXCORD_0_LOCATION,
			&attribute_buffer_views.texcoord_0,
			buffers,
		);
		Self::_bind_view(
			gl,
			attribute_locations::TEXCORD_1_LOCATION,
			&attribute_buffer_views.texcoord_1,
			buffers,
		);
		Self::_bind_view(
			gl,
			attribute_locations::TEXCORD_2_LOCATION,
			&attribute_buffer_views.texcoord_2,
			buffers,
		);
		Self::_bind_view(
			gl,
			attribute_locations::TEXCORD_3_LOCATION,
			&attribute_buffer_views.texcoord_3,
			buffers,
		);

		gl.bind_vertex_array(None);
		Ok(vao)
	}
	fn _bind_view(
		gl: &WebGl2RenderingContext,
		attribute_location: u32,
		view: &Option<BufferView>,
		buffers: &[Rc<Buffer>],
	) {
		if let Some(view) = view {
			buffers[view.buffer].bind(gl);
			gl.vertex_attrib_pointer_with_i32(
				attribute_location,
				view.component_size,
				view.component_type,
				false,
				view.stride,
				view.combined_offset,
			);
			gl.enable_vertex_attrib_array(attribute_location);
		}
	}
}
impl Drawable for Mesh {
	fn draw(&self, gl: &WebGl2RenderingContext, program: &Program) {
		self.material
			.bind_to_uniforms(gl, &program.uniform_locations);

		gl.bind_vertex_array(Some(&self.vao));

		if let Some(ref index_view) = self.index_view {
			self.buffers[index_view.buffer].bind(gl);
			gl.draw_elements_with_i32(
				self.mode,
				index_view.component_count,
				index_view.component_type,
				0, // index_view.combined_offset,
			);
		} else {
			gl.draw_arrays(
				self.mode,
				self.attribute_buffer_views.position.buffer_offset,
				self.attribute_buffer_views.position.component_count,
			);
		}

		gl.bind_vertex_array(None);
	}
}
