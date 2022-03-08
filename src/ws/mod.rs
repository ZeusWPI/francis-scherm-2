//! All handlers listenging to websocket based messages

use std::sync::Arc;

use actix::{ActorContext, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use super::AppState;

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AppState {
	fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		let msg = match msg {
			Err(_) => {
				ctx.stop();
				return;
			},
			Ok(msg) => msg,
		};

		match msg {
			ws::Message::Ping(msg) => ctx.pong(&msg),
			ws::Message::Pong(_) => (),
			ws::Message::Text(txt) => {
				let m = txt.trim();
				// Message format:
				// x y r g b
				// eg.
				// 10 10 255 120 120 -> set (10, 10) to #FF7878
				let parts = m.split(' ').collect::<Vec<&str>>();

				if parts.len() != 5 {
					ctx.text("ERROR (bad format)
						Message format is `x y r g b`
						Where:
						  - x: int
						  - y: int
						  - r: int8
						  - g: int8
						  - b: int8
						");
					return;
				}

				let x = parts[0].parse::<u32>().unwrap();
				let y = parts[1].parse::<u32>().unwrap();
				let r = parts[2].parse::<u8>().unwrap();
				let g = parts[3].parse::<u8>().unwrap();
				let b = parts[4].parse::<u8>().unwrap();

				let start_index_ul = (y * 2 * self.line_length
					+ x * 2 * self.bytes_per_pixel) as usize;

				let start_index_ur = (y * 2 * self.line_length
					+ (x * 2 + 1) * self.bytes_per_pixel) as usize;

				let start_index_ll = ((y * 2 + 1) * self.line_length
					+ x * 2 * self.bytes_per_pixel) as usize;

				let start_index_lr = ((y * 2 + 1) * self.line_length
					+ (x * 2 + 1) * self.bytes_per_pixel) as usize;

				let mut frame = self.frame.lock().unwrap();

				if start_index_lr + 2 < frame.len() {
					frame[start_index_ul] = b;
					frame[start_index_ul + 1] = g;
					frame[start_index_ul + 2] = r;

					frame[start_index_ur] = b;
					frame[start_index_ur + 1] = g;
					frame[start_index_ur + 2] = r;

					frame[start_index_ll] = b;
					frame[start_index_ll + 1] = g;
					frame[start_index_ll + 2] = r;

					frame[start_index_lr] = b;
					frame[start_index_lr + 1] = g;
					frame[start_index_lr + 2] = r;

					drop(frame);
					ctx.text("OK");
				} else {
					ctx.text("ERROR (out of bounds)");
				}
			},
			ws::Message::Binary(_) => ctx.text("unexpected binary"),
			ws::Message::Close(reason) => {
				ctx.close(reason);
				ctx.stop();
			},
			ws::Message::Continuation(_) => {
				ctx.stop();
			},
			ws::Message::Nop => (),
		}
	}
}

#[get("/set_pixel")]
async fn set_pixel(
	req: HttpRequest,
	stream: web::Payload,
	data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
	ws::start(
		AppState {
			line_length:     data.line_length,
			bytes_per_pixel: data.bytes_per_pixel,
			frame:           Arc::clone(&data.frame),
		},
		&req,
		stream,
	)
}