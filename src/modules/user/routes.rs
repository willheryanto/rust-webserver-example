//use actix_web::{get, web, HttpResponse, Responder};
//use sqlx::{PgPool};

//use crate::modules::user::{User};

//pub fn init(cfg: &mut web::ServiceConfig) {
    //cfg.service(find);
//}

//#[get("/session")]
//async fn find(pool: web::Data<PgPool>) -> impl Responder {
    //let res = User::find_by_id(id.into_inner(), &pool).await;
    //match res {
        //Ok(user) => HttpResponse::Ok().json(user),
        //Ok(None) => HttpResponse::NotFound().body("User not found"),
        //Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    //}
//}
