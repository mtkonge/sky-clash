use std::{ops::Deref, sync::Arc};

use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Serialize;

use crate::{
    board::Board,
    database::{CreateHeroParams, Database},
    BoardState, DbParam,
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

#[post("/update_heroes_on_board")]
pub async fn update_heroes_on_board(
    board_state: Data<BoardState>,
    req_body: Json<Board>,
) -> impl Responder {
    board_state.lock().await.hero_1_rfid = req_body.0.hero_1_rfid;
    board_state.lock().await.hero_2_rfid = req_body.0.hero_2_rfid;
    HttpResponse::Ok()
}

#[get("heroes_on_board")]
pub async fn heroes_on_board(board_state: Data<BoardState>) -> impl Responder {
    HttpResponse::Ok().json(board_state.lock().await.clone())
}
