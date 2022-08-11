use std::thread;
use std::time::Duration;

use actix_web::web::PathConfig;
use actix_web::{web, App, HttpResponse, HttpServer};
use framebuffer::Framebuffer;
use francis_scherm_2::{http, ws, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();

	let line_length = framebuffer.fix_screen_info.line_length;
	let bytes_per_pixel = framebuffer.var_screen_info.bits_per_pixel / 8;
	let size = (line_length * framebuffer.var_screen_info.yres) as usize;

	// Will be sent to the request handlers
	let mut frame = Box::pin(vec![0u8; size]);
	let frame_ptr = frame.as_mut_ptr() as usize; // Cast to usize so it's thread-safe hehe

	thread::spawn(move || {
		let frame = unsafe { std::slice::from_raw_parts(frame_ptr as *const u8, size) };

		loop {
			framebuffer.write_frame(frame);

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
			.app_data(web::Data::new(AppState { line_length, bytes_per_pixel, size, frame_ptr }))
			.service(ws::set_pixel)
			.service(http::set_pixel_path)
	})
	.bind(("0.0.0.0", 8000))?
	.run()
	.await
}
