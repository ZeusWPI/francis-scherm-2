//! Http listener

use actix_web::{post, web, HttpResponse, Responder};

use super::AppState;

/// Set a pixel based on path parameters
///
/// Endpoint: /{x}/{y}/{r}/{g}/{b}
#[post("/{x}/{y}/{r}/{g}/{b}/{a}")]
async fn set_pixel_path(
	params: web::Path<(u32, u32, u8, u8, u8, u8)>,
	data: web::Data<AppState>,
) -> impl Responder {
	let (x, y, r, g, b, a) = params.into_inner();

	match data.set_pixel(x, y, r, g, b, a) {
		Ok(_) => HttpResponse::Ok(),
		Err(_) => HttpResponse::NotFound(),
	}
}
