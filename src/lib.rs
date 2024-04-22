use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn healthcheck_route() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe_route(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(healthcheck_route))
            .route("/subscribe", web::post().to(subscribe_route))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
