mod database;
mod routes;
mod sqlite3_db;

use actix_web::{web, App, HttpServer};
use routes::{create_hero, hello};
use sqlite3_db::Sqlite3Db;
use tokio::sync::Mutex;

type DbParam = Mutex<Sqlite3Db>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Mutex::new(Sqlite3Db::new())))
            .service(create_hero)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
