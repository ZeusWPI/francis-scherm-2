use framebuffer::Framebuffer;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use actix_web::middleware::Logger;

use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

struct AppState {
    framebuffer: Arc<Mutex<Framebuffer>>,
    frame: Arc<Mutex<Vec<u8>>>,
}

#[post("/{x}/{y}/{r}/{g}/{b}")]
async fn set_pixel(params: web::Path<(u32, u32, u8, u8, u8)>, data: web::Data<AppState>) -> impl Responder {
    let framebuffer = data.framebuffer.lock().unwrap();
    let mut frame = data.frame.lock().unwrap();

    let line_length = framebuffer.fix_screen_info.line_length;
    let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;

    let x = params.0;
    let y = params.1;
    let r = params.2;
    let g = params.3;
    let b = params.4;

    let start_index = (y * line_length + x * bytespp) as usize;

    if start_index+2 < frame.len() {
        frame[start_index] = b;
        frame[start_index+1] = g;
        frame[start_index+2] = r;
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let actual_framebuffer = Framebuffer::new("/dev/fb0").unwrap();

    let h = actual_framebuffer.var_screen_info.yres;
    let line_length = actual_framebuffer.fix_screen_info.line_length;

    let framebuffer = Arc::new(Mutex::new(actual_framebuffer));
    let frame = Arc::new(Mutex::new(vec![0u8; (line_length * h) as usize]));

    let draw_framebuffer = framebuffer.clone();
    let draw_frame = frame.clone();

    thread::spawn(move || {
        loop {
            // Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();
            {
                let mut framebuffer = draw_framebuffer.lock().unwrap();
                let frame = draw_frame.lock().unwrap();

                framebuffer.write_frame(&frame);
            }
            // Framebuffer::set_kd_mode(KdMode::Text).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                framebuffer: framebuffer.clone(),
                frame: frame.clone(),
            }))
            .service(set_pixel)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
