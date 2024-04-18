mod database;
mod routes;
mod sqlite3_db;

use actix_web::{middleware::Logger, web, App, HttpServer};
use routes::{create_hero, hello};
use sqlite3_db::Sqlite3Db;
use tokio::sync::Mutex;

pub type DbParam = Mutex<Sqlite3Db>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db_param = web::Data::new(Mutex::new(Sqlite3Db::new().await.unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(db_param.clone())
            .service(create_hero)
            .service(hello)
            .wrap(Logger::new(""))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
