use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr as ReprDeserialize, Serialize_repr as ReprSerialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeroStats {
    pub strength: u8,
    pub agility: u8,
    pub defence: u8,
}

#[repr(i64)]
#[derive(Clone, Debug, ReprDeserialize, ReprSerialize)]
pub enum HeroKind {
    Centrist = 0,
    Strong = 1,
    Speed = 2,
    Tankie = 3,
}

impl From<HeroKind> for HeroStats {
    fn from(value: HeroKind) -> Self {
        Self::from(&value)
    }
}

impl From<&HeroKind> for HeroStats {
    fn from(value: &HeroKind) -> Self {
        use HeroKind::*;
        match value {
            Centrist => HeroStats {
                strength: 8,
                agility: 8,
                defence: 8,
            },
            Strong => HeroStats {
                strength: 12,
                agility: 8,
                defence: 4,
            },
            Speed => HeroStats {
                strength: 4,
                agility: 12,
                defence: 8,
            },
            Tankie => HeroStats {
                strength: 8,
                agility: 4,
                defence: 12,
            },
        }
    }
}

impl From<i64> for HeroKind {
    fn from(value: i64) -> Self {
        use HeroKind::*;
        match value {
            0 => Centrist,
            1 => Strong,
            2 => Speed,
            3 => Tankie,
            _ => panic!("expected correct value"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hero {
    pub id: i64,
    pub rfid: String,
    pub level: i64,
    pub hero_type: HeroKind,
    pub unallocated_skillpoints: i64,
    pub strength_points: i64,
    pub agility_points: i64,
    pub defence_points: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateHeroParams {
    pub rfid: String,
    pub hero_type: HeroKind,
    pub base_stats: HeroStats,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateHeroStatsParams {
    pub rfid: String,
    pub stats: HeroStats,
}
