use engine::Component;

#[derive(Clone)]
pub enum PlayerKind {
    Left,
    Right,
}

#[derive(Clone, Component)]
pub struct Player {
    pub kind: PlayerKind,
    pub hero: shared::Hero,
    pub knockback_modifier: f64,
    pub lives: i8,
}

impl Player {
    pub fn is_alive(&self) -> bool {
        self.lives > 0
    }

    pub fn is_dead(&self) -> bool {
        self.lives <= 0
    }
}
