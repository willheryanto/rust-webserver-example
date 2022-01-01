use actix_web::{get, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(liveness).service(readiness);
}

#[get("/healthcheck/liveness")]
async fn liveness() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[get("/healthcheck/readiness")]
async fn readiness() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}
