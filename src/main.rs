use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct LogData {
    level: String,
    event: String,
    message: String
}

fn log(log: web::Json<LogData>) -> impl Responder {
    println!("[{}] {}: {}", log.level, log.event, log.message);
    HttpResponse::Accepted()
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/log", web::post().to(log))
    })
    .bind("0.0.0.0:4800")
    .unwrap()
    .run()
    .unwrap();
}

