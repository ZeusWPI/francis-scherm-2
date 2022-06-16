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
				let old_b = frame[idx];
				let old_g = frame[idx + 1];
				let old_r = frame[idx + 2];

				frame[idx] = b * a + old_b * (255 - a);
				frame[idx + 1] = r * a + old_r * (255 - a);
				frame[idx + 2] = g * a + old_g * (255 - a);
			}

			Ok(())
		} else {
			Err("out of bounds".to_string())
		}
	}
}
