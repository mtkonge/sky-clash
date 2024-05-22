use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::server::{Board, HeroResult, Res, ServerStrategy};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MockConnection {
    id_counter: i64,
    heroes: HashMap<String, shared::Hero>,
    rfid_1: Option<String>,
    rfid_2: Option<String>,
}

impl MockConnection {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            heroes: HashMap::new(),
            rfid_1: None,
            rfid_2: None,
        }
    }
}

impl MockConnection {
    fn save(&mut self) {
        let json = serde_json::to_string(self).unwrap();
        std::fs::write("mock_db.json", json).unwrap();
    }

    fn load(&mut self) {
        let json = std::fs::read_to_string("mock_db.json").unwrap();
        *self = serde_json::from_str(&json).unwrap();
    }
}

#[derive(Clone)]
struct BoardStatusRes(Board);

impl Res<Board> for BoardStatusRes {
    fn try_receive(&mut self) -> Option<Board> {
        Some(self.0.clone())
    }
}

impl ServerStrategy for MockConnection {
    fn quit(&mut self) {
        // nothing
    }

    fn update_hero_stats(&mut self, params: shared::UpdateHeroStatsParams) {
        self.load();
        let Some(hero) = self.heroes.get_mut(&params.rfid) else {
            return;
        };
        hero.set_stats(params.stats);
        self.save();
    }

    fn create_hero(&mut self, params: shared::CreateHeroParams) {
        self.load();
        let id = self.id_counter;
        self.id_counter += 1;
        self.heroes.insert(
            params.rfid.clone(),
            shared::Hero {
                id,
                kind: params.hero_type,
                rfid: params.rfid,
                level: 0,
                strength_points: params.base_stats.strength,
                agility_points: params.base_stats.agility,
                defence_points: params.base_stats.defence,
            },
        );
        self.save();
    }

    fn board_status(&mut self) -> Box<dyn Res<Board>> {
        self.load();
        Box::new(BoardStatusRes(Board {
            hero_1: match &self.rfid_1 {
                Some(rfid) => Some(match self.heroes.get(rfid) {
                    Some(hero) => HeroResult::Hero(hero.clone()),
                    None => HeroResult::UnknownRfid(rfid.clone()),
                }),
                None => None,
            },
            hero_2: match &self.rfid_2 {
                Some(rfid) => Some(match self.heroes.get(rfid) {
                    Some(hero) => HeroResult::Hero(hero.clone()),
                    None => HeroResult::UnknownRfid(rfid.clone()),
                }),
                None => None,
            },
        }))
    }
}
