//! All handlers listenging to http based messages

use actix_web::{post, web, HttpResponse, Responder};

use super::AppState;

/// Set a pixel based on path parameters
///
/// Endpoint: /{x}/{y}/{r}/{g}/{b}
#[post("/{x}/{y}/{r}/{g}/{b}")]
async fn set_pixel_path(
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
