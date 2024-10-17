use super::log_request;
use super::AppState;
use actix_web::{get, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}

#[get("/ping")]
async fn ping(app_state: web::Data<AppState>) -> impl Responder {
    log_request("GET: /ping", &app_state.connections);

    HttpResponse::Ok().body("Pong!")
}