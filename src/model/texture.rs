// File: src/model/texture.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use std::rc::Rc;

use serde::Deserialize;
use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Sampler {
	mag_filter: i32,
	min_filter: i32,
	wrap_s: i32,
	wrap_t: i32,
}
impl Default for Sampler {
	fn default() -> Self {
		Sampler {
			mag_filter: WebGl2RenderingContext::LINEAR as i32,
			min_filter: WebGl2RenderingContext::LINEAR as i32,
			wrap_s: WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
			wrap_t: WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Texture {
	handle: WebGlTexture,
	sampler: Rc<Sampler>,
}
impl Texture {
	pub fn bind(&self, gl: &WebGl2RenderingContext, texture_unit: u32) {
		gl.active_texture(WebGl2RenderingContext::TEXTURE0 + texture_unit);
		gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.handle));
	}
	pub fn new(
		gl: &WebGl2RenderingContext,
		image_element: &HtmlImageElement,
		texture_unit: u32,
		sampler: &Rc<Sampler>,
	) -> Result<Self, &'static str> {
		let handle = gl.create_texture().ok_or("Failed to create a texture")?;

		gl.active_texture(WebGl2RenderingContext::TEXTURE0 + texture_unit);
		gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&handle));

		let (mip_level, use_mipmap, min_filter, fmt) = {
			let width = image_element.width();
			let height = image_element.height();
			let is_power_of_2 = Self::is_power_of_2(width) && Self::is_power_of_2(height);

			// let min_filter = sampler.min_filter as u32;
			// let has_mipmap_min_filter = min_filter == WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR
			// 	|| min_filter == WebGl2RenderingContext::LINEAR_MIPMAP_NEAREST
			// 	|| min_filter == WebGl2RenderingContext::NEAREST_MIPMAP_LINEAR
			// 	|| min_filter == WebGl2RenderingContext::NEAREST_MIPMAP_NEAREST;

			let use_mipmap = is_power_of_2; // && has_mipmap_min_filter;
			let min_filter = if use_mipmap {
				WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32
			} else {
				sampler.min_filter
			};

			let fmt = if texture_unit == 0 {
				WebGl2RenderingContext::RGBA
			} else {
				WebGl2RenderingContext::RGB
			};
			(0, use_mipmap, min_filter, fmt)
		};

		gl.tex_parameteri(
			WebGl2RenderingContext::TEXTURE_2D,
			WebGl2RenderingContext::TEXTURE_WRAP_S,
			sampler.wrap_s,
		);
		gl.tex_parameteri(
			WebGl2RenderingContext::TEXTURE_2D,
			WebGl2RenderingContext::TEXTURE_WRAP_T,
			sampler.wrap_t,
		);
		gl.tex_parameteri(
			WebGl2RenderingContext::TEXTURE_2D,
			WebGl2RenderingContext::TEXTURE_MIN_FILTER,
			min_filter,
		);
		gl.tex_parameteri(
			WebGl2RenderingContext::TEXTURE_2D,
			WebGl2RenderingContext::TEXTURE_MAG_FILTER,
			sampler.mag_filter,
		);

		let internal_format = fmt as i32;
		let src_format = fmt;
		let src_type = WebGl2RenderingContext::UNSIGNED_BYTE;
		gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
			WebGl2RenderingContext::TEXTURE_2D,
			mip_level,
			internal_format,
			src_format,
			src_type,
			image_element,
		)
		.map_err(|_e| "failed to create glTexture from image")?;

		if use_mipmap {
			gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
		}

		Ok(Self {
			handle,
			sampler: Rc::clone(sampler),
		})
	}
	pub fn is_power_of_2(num: u32) -> bool {
		matches!(
			num,
			2 | 4 | 8 | 16 | 32 | 64 | 128 | 256 | 512 | 1024 | 2048 | 4096
		)
	}
}
