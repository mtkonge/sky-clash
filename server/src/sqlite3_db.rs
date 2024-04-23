use std::env;

use eyre::eyre;
use eyre::Context;
use sqlx::SqlitePool;

use crate::database::Hero;
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
            "INSERT INTO heroes (rfid, level, hero_type, unallocated_skillpoints, strength_points, agility_points, defence_points) VALUES (?, 0, ?, 0, 0, 0, 0);",
            hero.rfid ,hero.hero_type
        )
        .execute(&self.pool)
        .await
        .with_context(|| "could not create hero in database")?;
        Ok(())
    }

    async fn hero_by_rfid(
        &mut self,
        rfid: &str,
    ) -> Result<Option<crate::database::Hero>, eyre::Report> {
        let result = sqlx::query_as!(Hero, "SELECT * FROM heroes WHERE rfid=?", rfid)
            .fetch_optional(&self.pool)
            .await;
        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(eyre!("Server error")),
        }
    }
}
