//! Websocket listener

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

				if !(parts.len() == 5 || parts.len() == 6) {
					ctx.text(
						"ERROR (bad format)
						Message format is `x y r g b [a]`
						Where:
						  - x: int
						  - y: int
						  - r: int8
						  - g: int8
						  - b: int8
						  - [a: int8]
						",
					);
					return;
				}

				let x = parts[0].parse::<u32>().unwrap();
				let y = parts[1].parse::<u32>().unwrap();
				let r = parts[2].parse::<u8>().unwrap();
				let g = parts[3].parse::<u8>().unwrap();
				let b = parts[4].parse::<u8>().unwrap();
				let mut a = 255u8;

				if parts.len() == 6 {
					a = parts[5].parse::<u8>().unwrap();
				}

				match self.set_pixel(x, y, r, g, b, a) {
					Ok(_) => (),
					Err(e) => ctx.text(e),
				};
			},
			ws::Message::Binary(bin) => {
				// XX XX XX XX YY YY YY YY RR GG BB AA
				if !(bin.len() == 11 || bin.len() == 12) {
					ctx.text(
						"ERROR (bad format)
						Message format is `x y r g b [a]`
						Where:
						  - x: uint32 (BE)
						  - y: uint32 (BE)
						  - r: uint8
						  - g: uint8
						  - b: uint8
						  - [a: uint8]
						",
					);
					return;
				}

				let x = u32::from_be_bytes(bin[0..4].try_into().unwrap());
				let y = u32::from_be_bytes(bin[4..8].try_into().unwrap());
				let r = bin[8];
				let g = bin[9];
				let b = bin[10];
				let mut a = 255u8;

				if bin.len() == 12 {
					a = bin[11];
				}

				match self.set_pixel(x, y, r, g, b, a) {
					Ok(_) => (),
					Err(e) => ctx.text(e),
				};
			},
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
			size:            data.size,
			frame_ptr:       data.frame_ptr,
		},
		&req,
		stream,
	)
}
