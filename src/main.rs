use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use actix::{Actor, ActorContext, StreamHandler};
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use framebuffer::Framebuffer;

struct AppState {
	line_length:     u32,
	bytes_per_pixel: u32,
	frame:           Arc<Mutex<Vec<u8>>>,
}

impl Actor for AppState {
	type Context = ws::WebsocketContext<Self>;
}

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

				let start_index =
					(y * self.line_length + x * self.bytes_per_pixel) as usize;

				let mut frame = self.frame.lock().unwrap();

				if start_index + 2 < frame.len() {
					frame[start_index] = b;
					frame[start_index + 1] = g;
					frame[start_index + 2] = r;

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
async fn set_pixel_ws(
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

#[post("/{x}/{y}/{r}/{g}/{b}")]
async fn set_pixel(
	params: web::Path<(u32, u32, u8, u8, u8)>,
	data: web::Data<AppState>,
) -> impl Responder {
	let mut frame = data.frame.lock().unwrap();

	let (x, y, r, g, b) = params.into_inner();

	let start_index = (y * data.line_length + x * data.bytes_per_pixel) as usize;

	if start_index + 2 < frame.len() {
		frame[start_index] = b;
		frame[start_index + 1] = g;
		frame[start_index + 2] = r;
		HttpResponse::Ok()
	} else {
		HttpResponse::NotFound()
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let actual_framebuffer = Framebuffer::new("/dev/fb0").unwrap();

	let height = actual_framebuffer.var_screen_info.yres;
	let line_length = actual_framebuffer.fix_screen_info.line_length;
	let bytes_per_pixel = actual_framebuffer.var_screen_info.bits_per_pixel / 8;

	// Will be sent to the request handler
	let frame = Arc::new(Mutex::new(vec![0u8; (line_length * height) as usize]));

	// Will be sent to the draw thread
	let draw_framebuffer = Arc::new(Mutex::new(actual_framebuffer));
	let draw_frame = Arc::clone(&frame);

	thread::spawn(move || {
		loop {
			// Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();

			let mut framebuffer = draw_framebuffer.lock().unwrap();
			let frame = draw_frame.lock().unwrap();

			framebuffer.write_frame(&frame);

			// Frame must be dropped so set_pixel can access it
			drop(frame);

			// Framebuffer::set_kd_mode(KdMode::Text).unwrap();
			thread::sleep(Duration::from_millis(20));
		}
	});

	HttpServer::new(move || {
		App::new()
			.wrap(Logger::default())
			.app_data(web::Data::new(AppState {
				line_length,
				bytes_per_pixel,
				frame: frame.clone(),
			}))
			.service(set_pixel_ws)
			.service(set_pixel)
	})
	.bind(("0.0.0.0", 8000))?
	.run()
	.await
}
