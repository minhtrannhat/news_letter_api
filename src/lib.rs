use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;

async fn healthcheck_route() -> HttpResponse {
    return HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error>{
    let server = HttpServer::new(||{
        App::new()
        .route("/health_check", web::get().to(healthcheck_route))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}
