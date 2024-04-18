use std::env;

use eyre::Context;
use sqlx::SqlitePool;

use crate::database::{CreateHeroParams, Database};

pub struct Sqlite3Db {
    pool: SqlitePool,
}

impl Sqlite3Db {
    pub async fn new() -> Result<Self, eyre::Report> {
        let pool = SqlitePool::connect(
            &env::var("DATABASE_URL").with_context(|| "unable to find DATABASE_URL in .env")?,
        )
        .await
        .with_context(|| "unable to connect to database")?;
        Ok(Self { pool })
    }
}

impl Database for Sqlite3Db {
    async fn create_hero(&mut self, hero: CreateHeroParams) -> Result<(), eyre::Report> {
        sqlx::query!(
            "INSERT INTO heroes (rfid, hero_type) VALUES (?, ?);",
            hero.rfid,
            hero.hero_type
        )
        .execute(&self.pool)
        .await
        .with_context(|| "Could not create hero in database")?;
        Ok(())
    }
}
