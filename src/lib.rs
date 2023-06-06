use actix_web::{dev::Server, get, post, App, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    return HttpResponse::Ok();
}

#[derive(serde::Deserialize)]
struct SubscribeFormData {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(_form: web::Form<SubscribeFormData>) -> impl Responder {
    return HttpResponse::Ok();
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();

    return Ok(server);
}
