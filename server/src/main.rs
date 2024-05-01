mod board;
mod database;
mod routes;
mod sqlite3_db;

use actix_web::{middleware::Logger, web, App, HttpServer};
use board::Board;
use routes::{create_hero, get_hero, heroes_on_board, update_heroes_on_board};
use sqlite3_db::Sqlite3Db;
use tokio::sync::Mutex;

pub type DbParam = Mutex<Sqlite3Db>;
pub type BoardState = Mutex<Board>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(err) = dotenv::dotenv() {
        println!("issue importing .env: {err}");
    };
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db_param = web::Data::new(Mutex::new(Sqlite3Db::new().await.unwrap()));
    let board_state = web::Data::new(Mutex::new(Board::new(None, None)));

    HttpServer::new(move || {
        App::new()
            .app_data(db_param.clone())
            .app_data(board_state.clone())
            .service(create_hero)
            .service(update_heroes_on_board)
            .service(heroes_on_board)
            .service(get_hero)
            .wrap(Logger::new(""))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
