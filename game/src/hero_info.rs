use std::path::PathBuf;

pub struct HeroStats {
    strength: u8,
    agility: u8,
    defence: u8,
}

#[repr(i64)]
#[derive(serde_repr::Serialize_repr, Clone)]
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
        match value {
            HeroType::Centrist => HeroInfo {
                base_stats: HeroStats {
                    strength: 8,
                    agility: 8,
                    defence: 8,
                },
                texture_path: PathBuf::from("./textures/sprites/centrist.png"),
            },
            HeroType::Strong => HeroInfo {
                base_stats: HeroStats {
                    strength: 12,
                    agility: 8,
                    defence: 4,
                },
                texture_path: PathBuf::from("./textures/sprites/strong.png"),
            },
            HeroType::Speed => HeroInfo {
                base_stats: HeroStats {
                    strength: 4,
                    agility: 12,
                    defence: 8,
                },
                texture_path: PathBuf::from("./textures/sprites/speed.png"),
            },
            HeroType::Tankie => HeroInfo {
                base_stats: HeroStats {
                    strength: 8,
                    agility: 4,
                    defence: 12,
                },
                texture_path: PathBuf::from("./textures/sprites/tankie.png"),
            },
        }
    }
}
