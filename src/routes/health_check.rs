use actix_web::HttpResponse;

pub async fn healthcheck_route() -> HttpResponse {
    HttpResponse::Ok().finish()
}
