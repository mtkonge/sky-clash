use std::path::PathBuf;

#[derive(serde::Serialize)]
pub struct HeroStats {
    pub strength: u8,
    pub agility: u8,
    pub defence: u8,
}

impl From<HeroType> for HeroStats {
    fn from(value: HeroType) -> Self {
        Self::from(&value)
    }
}

impl From<&HeroType> for HeroStats {
    fn from(value: &HeroType) -> Self {
        match value {
            HeroType::Centrist => HeroStats {
                strength: 8,
                agility: 8,
                defence: 8,
            },
            HeroType::Strong => HeroStats {
                strength: 12,
                agility: 8,
                defence: 4,
            },
            HeroType::Speed => HeroStats {
                strength: 4,
                agility: 12,
                defence: 8,
            },
            HeroType::Tankie => HeroStats {
                strength: 8,
                agility: 4,
                defence: 12,
            },
        }
    }
}

#[repr(i64)]
#[derive(Debug, serde_repr::Deserialize_repr, serde_repr::Serialize_repr, Clone)]
pub enum HeroType {
    Centrist = 0,
    Strong = 1,
    Speed = 2,
    Tankie = 3,
}

pub struct HeroInfo {
    pub base_stats: HeroStats,
    pub texture_path: PathBuf,
}

impl From<HeroType> for HeroInfo {
    fn from(value: HeroType) -> Self {
        Self::from(&value)
    }
}

impl From<&HeroType> for HeroInfo {
    fn from(value: &HeroType) -> Self {
        let base_stats = HeroStats::from(value);
        let texture_path = match value {
            HeroType::Centrist => PathBuf::from("./textures/sprites/centrist.png"),
            HeroType::Strong => PathBuf::from("./textures/sprites/strong.png"),
            HeroType::Speed => PathBuf::from("./textures/sprites/speed.png"),
            HeroType::Tankie => PathBuf::from("./textures/sprites/tankie.png"),
        };
        Self {
            base_stats,
            texture_path,
        }
    }
}
