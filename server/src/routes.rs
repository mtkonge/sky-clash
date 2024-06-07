use crate::{database::Database, BoardColors, BoardState, DbParam};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, HttpResponseBuilder, Responder,
};

#[post("/create_hero")]
pub async fn create_hero(
    db: Data<DbParam>,
    req_body: Json<shared::CreateHeroParams>,
) -> impl Responder {
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

#[post("/update_hero_stats")]
pub async fn update_hero_stats(
    db: Data<DbParam>,
    req_body: Json<shared::UpdateHeroStatsParams>,
) -> impl Responder {
    match db.lock().await.hero_by_rfid(&req_body.0.rfid).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::BadRequest(),
        Err(_) => return HttpResponse::InternalServerError(),
    };
    match db.lock().await.update_hero_stats(req_body.0).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[get("/hero/{rfid}")]
pub async fn get_hero(db: Data<DbParam>, rfid: Path<String>) -> impl Responder {
    match db.lock().await.hero_by_rfid(rfid.clone().as_str()).await {
        Ok(Some(hero)) => HttpResponse::Ok().json(Some(hero)),
        Ok(None) => HttpResponse::NotFound().json(None::<()>),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/update_heroes_on_board")]
pub async fn update_heroes_on_board(
    board_state: Data<BoardState>,
    req_body: Json<shared::Board>,
) -> impl Responder {
    board_state.lock().await.hero_1_rfid = req_body.0.hero_1_rfid;
    board_state.lock().await.hero_2_rfid = req_body.0.hero_2_rfid;
    HttpResponse::Ok()
}

#[get("/heroes_on_board")]
pub async fn heroes_on_board(board_state: Data<BoardState>) -> impl Responder {
    HttpResponse::Ok().json(board_state.lock().await.clone())
}

#[post("/update_board_colors")]
pub async fn update_board_colors(
    board_colors: Data<BoardColors>,
    req_body: Json<shared::UpdateBoardColorsParams>,
) -> impl Responder {
    let Json(shared::UpdateBoardColorsParams {
        hero_1_color,
        hero_2_color,
    }) = req_body;
    board_colors.lock().await.0 = [hero_1_color, hero_2_color];
    HttpResponse::Ok()
}

#[get("/board_colors")]
pub async fn get_board_colors(board_colors: Data<BoardColors>) -> impl Responder {
    HttpResponse::Ok().json(board_colors.lock().await.0)
}

#[post("/create_match")]
pub async fn create_match(
    db: Data<DbParam>,
    req_json: Json<shared::CreateMatchParams>,
) -> impl Responder {
    let shared::CreateMatchParams {
        loser_hero_id,
        winner_hero_id,
    } = req_json.0;
    let _loser = match find_hero(db.clone(), loser_hero_id).await {
        Ok(player) => player,
        Err(res) => return res,
    };
    let winner = match find_hero(db.clone(), winner_hero_id).await {
        Ok(player) => player,
        Err(res) => return res,
    };
    match db
        .lock()
        .await
        .update_hero_level(winner.id, winner.level + 1)
        .await
    {
        Ok(_) => {}
        Err(_) => return HttpResponse::InternalServerError(),
    }
    match db
        .lock()
        .await
        .create_match(shared::CreateMatchParams {
            winner_hero_id,
            loser_hero_id,
        })
        .await
    {
        Ok(()) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

async fn find_hero(db: Data<DbParam>, id: i64) -> Result<shared::Hero, HttpResponseBuilder> {
    match db.lock().await.hero_by_id(id).await {
        Ok(Some(hero)) => Ok(hero),
        Ok(None) => Err(HttpResponse::BadRequest()),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}
