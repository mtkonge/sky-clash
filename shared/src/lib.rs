use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    pub hero_1_rfid: Option<String>,
    pub hero_2_rfid: Option<String>,
}

impl Board {
    pub fn new(hero_1_rfid: Option<String>, hero_2_rfid: Option<String>) -> Self {
        Self {
            hero_1_rfid,
            hero_2_rfid,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone)]
pub struct HeroStats {
    pub strength: i64,
    pub agility: i64,
    pub defence: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateHeroParams {
    pub rfid: String,
    pub hero_type: i64,
    pub base_stats: HeroStats,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateHeroStatsParams {
    pub rfid: String,
    pub stats: HeroStats,
}
