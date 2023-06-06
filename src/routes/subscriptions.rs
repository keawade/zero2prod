use actix_web::{post, web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

#[post("/subscribe")]
pub async fn subscribe(_form: web::Form<SubscribeFormData>) -> impl Responder {
    HttpResponse::Ok()
}
