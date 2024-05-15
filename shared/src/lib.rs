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
    pub strength: i64,
    pub agility: i64,
    pub defence: i64,
}

#[repr(i64)]
#[derive(Clone, Debug, ReprDeserialize, ReprSerialize)]
pub enum HeroKind {
    Centrist = 0,
    Strong = 1,
    Speed = 2,
    Tankie = 3,
}

impl std::fmt::Display for HeroKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            HeroKind::Centrist => "Centrist",
            HeroKind::Strong => "Strong",
            HeroKind::Speed => "Speed",
            HeroKind::Tankie => "Tankie",
        };
        write!(f, "{}", text)
    }
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
    pub strength_points: i64,
    pub agility_points: i64,
    pub defence_points: i64,
}

impl Hero {
    pub fn total_skill_points(&self) -> i64 {
        self.level * 3 + 24
    }
    pub fn unallocated_skill_points(&self) -> i64 {
        let total_allocated = self.strength_points + self.agility_points + self.defence_points;
        self.total_skill_points() - total_allocated
    }
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
