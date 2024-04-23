use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::{
    board::Board,
    database::{CreateHeroParams, Database},
    BoardState, DbParam,
};

#[post("/create_hero")]
pub async fn create_hero(db: Data<DbParam>, req_body: Json<CreateHeroParams>) -> impl Responder {
    match db.lock().await.hero_by_rfid(&req_body.0.rfid).await {
        Ok(Some(_)) => return HttpResponse::BadRequest(),
        Ok(None) => (),
        Err(_) => return HttpResponse::InternalServerError(),
    }
    match db.lock().await.create_hero(req_body.0).await {
        Ok(()) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[get("hero/{rfid}")]
pub async fn get_hero(db: Data<DbParam>, rfid: Path<String>) -> impl Responder {
    match db.lock().await.hero_by_rfid(rfid.clone().as_str()).await {
        Ok(Some(hero)) => HttpResponse::Ok().json(hero),
        Ok(None) => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
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
