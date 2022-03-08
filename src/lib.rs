//! Collection of different handlers for a variety of formats and protocols

use std::sync::{Arc, Mutex};

use actix::Actor;
use actix_web_actors;

pub mod http;
pub mod ws;

pub struct AppState {
	pub line_length:     u32,
	pub bytes_per_pixel: u32,
	pub frame:           Arc<Mutex<Vec<u8>>>,
}

impl Actor for AppState {
	type Context = actix_web_actors::ws::WebsocketContext<Self>;
}
