use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn healthcheck_route() -> impl Responder{
    return HttpResponse::Ok().finish()
}


pub async fn run() -> Result<(), std::io::Error>{
    HttpServer::new(||{
        App::new()
        .route("/healthcheck", web::get().to(healthcheck_route))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
