use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    database::{CreateHeroParams, Database},
    DbParam,
};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/create_hero")]
pub async fn create_hero(db: Data<DbParam>, req_body: Json<CreateHeroParams>) -> impl Responder {
    if (db.lock().await.hero_by_rfid(&req_body.0.rfid).await).is_ok() {
        return HttpResponse::Forbidden();
    }
    match db.lock().await.create_hero(req_body.0).await {
        Ok(()) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
