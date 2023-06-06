use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;

use super::routes::health_check;
use super::routes::subscribe;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();

    Ok(server)
}
