use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use actix_web::web::PathConfig;
use actix_web::{web, App, HttpResponse, HttpServer};
use framebuffer::Framebuffer;
use francis_scherm_2::{http, ws, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();

	let height = framebuffer.var_screen_info.yres;
	let line_length = framebuffer.fix_screen_info.line_length;
	let bytes_per_pixel = framebuffer.var_screen_info.bits_per_pixel / 8;

	// Will be sent to the request handler
	let frame = Arc::new(Mutex::new(vec![0u8; (line_length * height) as usize]));

	// Will be sent to the draw thread
	let draw_frame = Arc::clone(&frame);

	thread::spawn(move || {
		loop {
			let frame = draw_frame.lock().unwrap();

			framebuffer.write_frame(&frame);

			// Frame must be dropped so set_pixel can access it
			drop(frame);

			thread::sleep(Duration::from_millis(5));
		}
	});

	HttpServer::new(move || {
		App::new()
			.app_data(PathConfig::default().error_handler(|err, _req| {
				actix_web::error::InternalError::from_response(
					err,
					HttpResponse::BadRequest().into(),
				)
				.into()
			}))
			.app_data(web::Data::new(AppState {
				line_length,
				bytes_per_pixel,
				frame: frame.clone(),
			}))
			.service(ws::set_pixel)
			.service(http::set_pixel_path)
	})
	.bind(("0.0.0.0", 8000))?
	.run()
	.await
}
