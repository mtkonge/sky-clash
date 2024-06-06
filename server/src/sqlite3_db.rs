use std::env;

use eyre::eyre;
use eyre::Context;
use sqlx::SqlitePool;

use crate::database::Database;

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
    async fn create_hero(&mut self, hero: shared::CreateHeroParams) -> Result<(), eyre::Report> {
        let hero_type = hero.hero_type as i64;
        sqlx::query!(
            "INSERT INTO heroes (rfid, level, kind, strength_points, agility_points, defence_points) VALUES (?, 0, ?, ?, ?, ?);",
            hero.rfid, hero_type, hero.base_stats.strength, hero.base_stats.agility, hero.base_stats.defence,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "could not create hero in database")?;
        Ok(())
    }

    async fn hero_by_id(&mut self, id: i64) -> Result<Option<shared::Hero>, eyre::Report> {
        let result = sqlx::query_as!(shared::Hero, "SELECT * FROM heroes WHERE id=?", id)
            .fetch_optional(&self.pool)
            .await;
        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(eyre!("Server error")),
        }
    }

    async fn hero_by_rfid(&mut self, rfid: &str) -> Result<Option<shared::Hero>, eyre::Report> {
        let result = sqlx::query_as!(shared::Hero, "SELECT * FROM heroes WHERE rfid=?", rfid)
            .fetch_optional(&self.pool)
            .await;
        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(eyre!("Server error")),
        }
    }

    async fn update_hero_stats(
        &mut self,
        params: shared::UpdateHeroStatsParams,
    ) -> Result<(), eyre::Report> {
        sqlx::query!(
            "UPDATE heroes SET strength_points=?, agility_points=?, defence_points=? WHERE rfid = ?",
            params.stats.strength, params.stats.agility, params.stats.defence, params.rfid
        ).execute(&self.pool).await.with_context(|| "could not update hero stats")?;
        Ok(())
    }

    async fn update_hero_level(&mut self, id: i64, level: i64) -> Result<(), eyre::Report> {
        sqlx::query!("UPDATE heroes SET level=? WHERE id=?", level, id,)
            .execute(&self.pool)
            .await
            .with_context(|| "could not update hero level")?;
        Ok(())
    }

    async fn create_match(
        &mut self,
        params: shared::CreateMatchParams,
    ) -> Result<(), eyre::Report> {
        sqlx::query!(
            "INSERT INTO matches (loser, winner) VALUES (?, ?);",
            params.loser_hero_id,
            params.winner_hero_id,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "could not create match in database")?;
        Ok(())
    }
}
