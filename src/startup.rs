use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::{PgPool};
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check, subscribe, delete};

pub fn run_server(listener: TcpListener, conn_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(conn_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscription", web::post().to(subscribe))
            .route("/delete", web::post().to(delete))
            .app_data(db_pool.clone())

    })
        .listen(listener)?
        .run();


    Ok(server)
}