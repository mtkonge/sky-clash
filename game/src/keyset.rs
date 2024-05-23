use engine::Keycode;

#[derive(Clone)]
pub enum Keyset {
    Wasd,
    ArrowKeys,
}

impl Keyset {
    pub fn right(&self) -> Keycode {
        match self {
            Keyset::Wasd => Keycode::D,
            Keyset::ArrowKeys => Keycode::Right,
        }
    }
    pub fn left(&self) -> Keycode {
        match self {
            Keyset::Wasd => Keycode::A,
            Keyset::ArrowKeys => Keycode::Left,
        }
    }
    pub fn up(&self) -> Keycode {
        match self {
            Keyset::Wasd => Keycode::W,
            Keyset::ArrowKeys => Keycode::Up,
        }
    }
    pub fn down(&self) -> Keycode {
        match self {
            Keyset::Wasd => Keycode::S,
            Keyset::ArrowKeys => Keycode::Down,
        }
    }
    pub fn light_attack(&self) -> Keycode {
        match self {
            Keyset::Wasd => Keycode::J,
            Keyset::ArrowKeys => Keycode::KpEnter,
        }
    }

    pub fn dodge(&self) -> Keycode {
        match self {
            Keyset::Wasd => Keycode::K,
            Keyset::ArrowKeys => Keycode::KpPeriod,
        }
    }
}
