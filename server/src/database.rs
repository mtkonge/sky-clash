use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateHeroParams {
    pub rfid: String,
    pub hero_type: i64,
}

pub struct Hero {
    pub id: i64,
    pub rfid: String,
    pub level: i64,
    pub hero_type: i64,
    pub unallocated_skillpoints: i64,
    pub strength_points: i64,
    pub agility_points: i64,
    pub defence_points: i64,
}

pub trait Database {
    async fn create_hero(&mut self, hero: CreateHeroParams) -> Result<(), eyre::Report>;
    async fn hero_by_rfid(&mut self, rfid: &str) -> Result<Option<Hero>, eyre::Report>;
}
