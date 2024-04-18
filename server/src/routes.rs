use actix_web::{get, post, web::Data, HttpResponse, Responder};

use crate::{
    database::{CreateHeroParams, Database},
    DbParam,
};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/create_hero")]
pub async fn create_hero(
    db: Data<DbParam>,
    req_body: actix_web::web::Json<CreateHeroParams>,
) -> impl Responder {
    match db.lock().await.create_hero(req_body.0).await {
        Ok(()) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
