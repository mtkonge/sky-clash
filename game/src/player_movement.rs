use engine::{query, rigid_body::RigidBody, Collider, Component, System};

use crate::{hurtbox::Victim, keyset::Keyset};

#[derive(Clone)]
enum JumpState {
    OnGround,
    Jumped,
    DoubleJumped,
}

impl JumpState {
    pub fn next(&self) -> Self {
        match self {
            JumpState::OnGround => JumpState::Jumped,
            JumpState::Jumped => JumpState::DoubleJumped,
            JumpState::DoubleJumped => JumpState::DoubleJumped,
        }
    }

    pub fn can_jump(&self) -> bool {
        match self {
            JumpState::OnGround => true,
            JumpState::Jumped => true,
            JumpState::DoubleJumped => false,
        }
    }
}

#[derive(Component, Clone)]
pub struct PlayerMovement {
    keyset: Keyset,
    jump: JumpState,
}

impl PlayerMovement {
    pub fn new(keyset: Keyset) -> Self {
        Self {
            keyset,
            jump: JumpState::DoubleJumped,
        }
    }
}

pub struct PlayerMovementSystem(pub u64);
impl System for PlayerMovementSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, PlayerMovement, Victim, RigidBody, Collider) {
            let keyset = ctx.select::<PlayerMovement>(id).clone().keyset;
            let right_pressed = ctx.key_pressed(keyset.right());
            let left_pressed = ctx.key_pressed(keyset.left());
            let up_pressed = ctx.key_just_pressed(keyset.up());
            let down_pressed = ctx.key_pressed(keyset.down());
            let collider = ctx.select::<Collider>(id).clone();
            let victim = ctx.select::<Victim>(id).clone();
            let player_movement = ctx.select::<PlayerMovement>(id).clone();
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

            if up_pressed && player_movement.jump.can_jump() {
                body.vel.1 = -800.0;
                let player_movement = ctx.select::<PlayerMovement>(id);
                player_movement.jump = player_movement.jump.next();
            }

            if collider
                .colliding
                .is_some_and(|dir| dir.facing(engine::collision::Direction::Bottom))
            {
                let player_movement = ctx.select::<PlayerMovement>(id);
                player_movement.jump = JumpState::OnGround;
            }
        }
        Ok(())
    }
}
