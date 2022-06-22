//! Collection of different handlers for a variety of formats and protocols

use std::sync::{Arc, Mutex};

use actix::Actor;
use actix_web_actors::ws::WebsocketContext;

pub mod http;
pub mod ws;

pub struct AppState {
	pub line_length:     u32,
	pub bytes_per_pixel: u32,
	pub frame:           Arc<Mutex<Vec<u8>>>,
}

impl Actor for AppState {
	type Context = WebsocketContext<Self>;
}

impl AppState {
	#[inline(always)]
	pub fn set_pixel(&self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) -> Result<(), String> {
		let indices = [
			(y * 2 * self.line_length + x * 2 * self.bytes_per_pixel) as usize,
			(y * 2 * self.line_length + (x * 2 + 1) * self.bytes_per_pixel) as usize,
			((y * 2 + 1) * self.line_length + x * 2 * self.bytes_per_pixel) as usize,
			((y * 2 + 1) * self.line_length + (x * 2 + 1) * self.bytes_per_pixel) as usize,
		];

		let mut frame = self.frame.lock().unwrap();

		if indices[3] + 2 < frame.len() {
			for idx in indices {
				let old_b = frame[idx] as u32;
				let old_g = frame[idx + 1] as u32;
				let old_r = frame[idx + 2] as u32;

				frame[idx] = (((b as u32) * (a as u32) + old_b * (255 - (a as u32))) / 255) as u8;
				frame[idx + 1] =
					(((g as u32) * (a as u32) + old_g * (255 - (a as u32))) / 255) as u8;
				frame[idx + 2] =
					(((r as u32) * (a as u32) + old_r * (255 - (a as u32))) / 255) as u8;
			}

			Ok(())
		} else {
			Err("out of bounds".to_string())
		}
	}
}
