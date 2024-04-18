use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateHeroParams {
    pub rfid: String,
    pub hero_type: u32,
}

pub trait Database {
    async fn create_hero(&mut self, hero: CreateHeroParams) -> Result<(), eyre::Report>;
}
