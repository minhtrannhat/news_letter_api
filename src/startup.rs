use crate::routes::{healthcheck_route, subscribe_route};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_conn_pool: PgPool) -> Result<Server, std::io::Error> {
    // under the hood, web::Data::new will create an Arc
    // to make the TCP connection to PostgreSQL clone-able
    let db_conn_pool = web::Data::new(db_conn_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(healthcheck_route))
            .route("/subscriptions", web::post().to(subscribe_route))
            .app_data(db_conn_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
