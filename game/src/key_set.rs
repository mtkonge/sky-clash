use engine::Keycode;

#[derive(Clone)]
pub enum KeySet {
    Wasd,
    ArrowKeys,
}

impl KeySet {
    pub fn right(&self) -> Keycode {
        match self {
            KeySet::Wasd => Keycode::D,
            KeySet::ArrowKeys => Keycode::Right,
        }
    }
    pub fn left(&self) -> Keycode {
        match self {
            KeySet::Wasd => Keycode::A,
            KeySet::ArrowKeys => Keycode::Left,
        }
    }
    pub fn up(&self) -> Keycode {
        match self {
            KeySet::Wasd => Keycode::W,
            KeySet::ArrowKeys => Keycode::Up,
        }
    }
    pub fn down(&self) -> Keycode {
        match self {
            KeySet::Wasd => Keycode::S,
            KeySet::ArrowKeys => Keycode::Down,
        }
    }
    pub fn light_attack(&self) -> Keycode {
        match self {
            KeySet::Wasd => Keycode::Q,
            KeySet::ArrowKeys => Keycode::Delete,
        }
    }
}
