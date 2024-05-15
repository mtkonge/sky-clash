use engine::{query, rigid_body::RigidBody, Collider, Component, Keycode, System};

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
}

#[derive(Component, Clone)]
pub struct PlayerMovement {
    pub key_set: KeySet,
}

pub struct PlayerMovementSystem(pub u64);
impl System for PlayerMovementSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, PlayerMovement, RigidBody, Collider) {
            let key_set = ctx.select::<PlayerMovement>(id).clone().key_set;
            let right_pressed = ctx.key_pressed(key_set.right());
            let left_pressed = ctx.key_pressed(key_set.left());
            let up_pressed = ctx.key_pressed(key_set.up());
            let collider = ctx.select::<Collider>(id).clone();
            let body = ctx.select::<RigidBody>(id);

            if right_pressed && !left_pressed && body.vel.0 < 400.0 {
                body.vel.0 += 400.0 * delta * 8.0
            } else if left_pressed && !right_pressed && body.vel.0 > (-400.0) {
                body.vel.0 -= 400.0 * delta * 8.0
            }

            if collider
                .colliding
                .is_some_and(|dir| dir.facing(engine::collision::Direction::Bottom))
                && up_pressed
            {
                body.vel.1 = -800.0;
            }
        }
        Ok(())
    }
}
