use crate::routes::{healthcheck_route, subscribe_route};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

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
