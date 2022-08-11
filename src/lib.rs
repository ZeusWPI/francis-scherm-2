//! Collection of different handlers for a variety of formats and protocols

use actix::Actor;
use actix_web_actors::ws::WebsocketContext;

pub mod http;
pub mod ws;

pub struct AppState {
	pub line_length:     u32,
	pub bytes_per_pixel: u32,
	pub size:            usize,
	pub frame_ptr:       usize,
}

impl Actor for AppState {
	type Context = WebsocketContext<Self>;
}

impl AppState {
	#[inline(always)]
	pub fn set_pixel(&self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) -> Result<(), String> {
		let indices = [
			(y * 2 * self.line_length + x * 2 * self.bytes_per_pixel) as isize,
			(y * 2 * self.line_length + (x * 2 + 1) * self.bytes_per_pixel) as isize,
			((y * 2 + 1) * self.line_length + x * 2 * self.bytes_per_pixel) as isize,
			((y * 2 + 1) * self.line_length + (x * 2 + 1) * self.bytes_per_pixel) as isize,
		];

		let frame = self.frame_ptr as *mut u8;

		if indices[3] + (self.bytes_per_pixel as isize) > self.size as isize {
			return Err("out of bounds".to_string());
		}

		for idx in indices {
			unsafe {
				let old_b = *frame.offset(idx) as u16;
				let old_g = *frame.offset(idx + 1) as u16;
				let old_r = *frame.offset(idx + 2) as u16;

				let new_b = ((b as u16 * a as u16 + old_b * (255u16 - a as u16)) / 255u16) as u8;
				let new_g = ((g as u16 * a as u16 + old_g * (255u16 - a as u16)) / 255u16) as u8;
				let new_r = ((r as u16 * a as u16 + old_r * (255u16 - a as u16)) / 255u16) as u8;

				*frame.offset(idx) = new_b;
				*frame.offset(idx + 1) = new_g;
				*frame.offset(idx + 2) = new_r;
			}
		}

		Ok(())
	}
}
