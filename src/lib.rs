use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn healthcheck_route() -> HttpResponse {
    return HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error>{
    let server = HttpServer::new(||{
        App::new()
        .route("/health_check", web::get().to(healthcheck_route))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
