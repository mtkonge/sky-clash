use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
