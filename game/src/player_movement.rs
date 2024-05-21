use engine::{query, rigid_body::RigidBody, Collider, Component, System};

use crate::{hurtbox::Victim, key_set::KeySet};

#[derive(Component, Clone)]
pub struct PlayerMovement {
    pub key_set: KeySet,
}

pub struct PlayerMovementSystem(pub u64);
impl System for PlayerMovementSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, PlayerMovement, Victim, RigidBody, Collider) {
            let key_set = ctx.select::<PlayerMovement>(id).clone().key_set;
            let right_pressed = ctx.key_pressed(key_set.right());
            let left_pressed = ctx.key_pressed(key_set.left());
            let up_pressed = ctx.key_pressed(key_set.up());
            let down_pressed = ctx.key_pressed(key_set.down());
            let collider = ctx.select::<Collider>(id).clone();
            let victim = ctx.select::<Victim>(id).clone();
            let body = ctx.select::<RigidBody>(id);

            if victim.stunned.is_some() {
                continue;
            }

            if right_pressed && !left_pressed && body.vel.0 < 400.0 {
                body.vel.0 += 400.0 * delta * 8.0
            } else if left_pressed && !right_pressed && body.vel.0 > (-400.0) {
                body.vel.0 -= 400.0 * delta * 8.0
            }

            if down_pressed && body.vel.1 < 800.0 {
                body.vel.1 += 1600.0 * delta
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
