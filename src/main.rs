use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use framebuffer::Framebuffer;

struct AppState {
	line_length:     u32,
	bytes_per_pixel: u32,
	frame:           Arc<Mutex<Vec<u8>>>,
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
			.service(set_pixel)
	})
	.bind(("0.0.0.0", 8000))?
	.run()
	.await
}
