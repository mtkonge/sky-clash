use engine::{query, rigid_body::RigidBody, spawn, Collider, Component, System};

use crate::{
    hurtbox::{HurtDirection, Hurtbox, Victim},
    keyset::Keyset,
    sprite_renderer::Sprite,
};

enum AttackKind {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Clone)]
pub struct PlayerInteraction {
    pub keyset: Keyset,
    pub attack_cooldown: f64,
    pub jump_state: JumpState,
}

impl PlayerInteraction {
    pub fn new(keyset: Keyset, attack_cooldown: f64) -> Self {
        Self {
            keyset,
            attack_cooldown,
            jump_state: JumpState::DoubleJumped,
        }
    }

    pub fn can_jump(&self) -> bool {
        match self.jump_state {
            JumpState::OnGround => true,
            JumpState::Jumped => true,
            JumpState::DoubleJumped => false,
        }
    }
}

pub struct PlayerInteractionSystem(pub u64);
impl System for PlayerInteractionSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        self.update_player_attack(ctx, delta)
    }
}

impl PlayerInteractionSystem {
    fn spawn_attack(
        &self,
        ctx: &mut engine::Context,
        attack_kind: AttackKind,
        direction: HurtDirection,
        id: u64,
        body: &RigidBody,
    ) {
        let attack_size = self.attack_size(&attack_kind);
        let pos = self.attack_pos(&attack_kind, body, attack_size);
        let vel = self.attack_vel(&attack_kind, body.vel);
        let textures = self.attack_textures(ctx, &attack_kind);
        spawn!(
            ctx,
            Sprite::new(textures[0]),
            // .size((256.0, 64.0))
            // .offset((0.0, -16.0)),
            RigidBody::new()
                .with_pos(pos)
                .with_vel(vel)
                .with_size(attack_size),
            Hurtbox {
                direction,
                power: 20.0,
                owner: Some(id),
                duration: 0.3,
                stun_time: Some(0.5),
                textures,
                ..Default::default()
            }
        );
    }

    fn attack_size(&self, attack_kind: &AttackKind) -> (f64, f64) {
        match attack_kind {
            AttackKind::Up => (128.0, 64.0),
            AttackKind::Down => (128.0 * 2.0, 32.0),
            AttackKind::Left => (64.0, 128.0),
            AttackKind::Right => (64.0, 128.0),
        }
    }

    fn attack_pos(
        &self,
        attack_kind: &AttackKind,
        body: &RigidBody,
        attack_size: (f64, f64),
    ) -> (f64, f64) {
        match attack_kind {
            AttackKind::Up => (
                body.pos.0 + (body.size.0 - attack_size.0) / 2.0,
                body.pos.1 - attack_size.1,
            ),
            AttackKind::Down => (
                body.pos.0 + (body.size.0 - attack_size.0) / 2.0,
                body.pos.1 + body.size.1 - attack_size.1,
            ),
            AttackKind::Left => (
                body.pos.0 - attack_size.0,
                body.pos.1 + (body.size.1 - attack_size.1) / 2.0,
            ),
            AttackKind::Right => (
                body.pos.0 + body.size.0,
                body.pos.1 + (body.size.1 - attack_size.1) / 2.0,
            ),
        }
    }

    fn attack_vel(&self, attack_kind: &AttackKind, vel: (f64, f64)) -> (f64, f64) {
        match attack_kind {
            AttackKind::Up => (0.0, 0.0),
            AttackKind::Down => (0.0, 0.0),
            AttackKind::Left => (vel.0 / 2.0, vel.1 / 2.0),
            AttackKind::Right => (vel.0 / 2.0, vel.1 / 2.0),
        }
    }

    fn attack_textures(
        &self,
        ctx: &mut engine::Context,
        attack_kind: &AttackKind,
    ) -> Vec<engine::Texture> {
        match attack_kind {
            AttackKind::Up => vec![
                "textures/attacks/up_0.png".to_string(),
                "textures/attacks/up_1.png".to_string(),
                "textures/attacks/up_2.png".to_string(),
                "textures/attacks/up_3.png".to_string(),
                "textures/attacks/up_4.png".to_string(),
            ],
            AttackKind::Down => vec![
                "textures/attacks/down_0.png".to_string(),
                "textures/attacks/down_1.png".to_string(),
                "textures/attacks/down_2.png".to_string(),
                "textures/attacks/down_3.png".to_string(),
                "textures/attacks/down_4.png".to_string(),
                "textures/attacks/down_5.png".to_string(),
                "textures/attacks/down_6.png".to_string(),
                "textures/attacks/down_7.png".to_string(),
            ],
            AttackKind::Left => vec![
                "textures/attacks/left_0.png".to_string(),
                "textures/attacks/left_1.png".to_string(),
                "textures/attacks/left_2.png".to_string(),
                "textures/attacks/left_3.png".to_string(),
                "textures/attacks/left_4.png".to_string(),
            ],
            AttackKind::Right => vec![
                "textures/attacks/right_0.png".to_string(),
                "textures/attacks/right_1.png".to_string(),
                "textures/attacks/right_2.png".to_string(),
                "textures/attacks/right_3.png".to_string(),
                "textures/attacks/right_4.png".to_string(),
            ],
        }
        .into_iter()
        .map(|path| ctx.load_texture(path).unwrap())
        .collect::<Vec<_>>()
    }

    fn update_player_attack(
        &self,
        ctx: &mut engine::Context,
        delta: f64,
    ) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody, Collider, PlayerInteraction, Victim) {
            let player_attack = ctx.select::<PlayerInteraction>(id).clone();
            let keyset = player_attack.keyset;
            let right_pressed = ctx.key_pressed(keyset.right());
            let left_pressed = ctx.key_pressed(keyset.left());
            let down_pressed = ctx.key_pressed(keyset.down());
            let light_attack_pressed = ctx.key_just_pressed(keyset.light_attack());
            let victim = ctx.select::<Victim>(id).clone();
            let body = ctx.select::<RigidBody>(id).clone();

            if victim.stunned.is_some() {
                for hurtbox_id in query!(ctx, Hurtbox, RigidBody) {
                    let hurtbox = ctx.select::<Hurtbox>(hurtbox_id);
                    if hurtbox.owner.is_some_and(|owner| owner == id) {
                        hurtbox.duration_passed = hurtbox.duration
                    };
                }
                continue;
            }

            if player_attack.attack_cooldown >= 0.0 {
                let player_attack = ctx.select::<PlayerInteraction>(id);
                player_attack.attack_cooldown -= delta;
                continue;
            }

            if !light_attack_pressed {
                continue;
            }

            if down_pressed {
                self.spawn_attack(ctx, AttackKind::Down, HurtDirection::Up, id, &body)
            } else if left_pressed && !right_pressed {
                self.spawn_attack(ctx, AttackKind::Left, HurtDirection::Left, id, &body)
            } else if right_pressed && !left_pressed {
                self.spawn_attack(ctx, AttackKind::Right, HurtDirection::Right, id, &body)
            } else {
                self.spawn_attack(ctx, AttackKind::Up, HurtDirection::Up, id, &body)
            }
            let player_attack = ctx.select::<PlayerInteraction>(id);
            player_attack.attack_cooldown = 0.5;
        }

        Ok(())
    }

    fn update_player_movement(
        &self,
        ctx: &mut engine::Context,
        delta: f64,
    ) -> Result<(), engine::Error> {
        for id in query!(ctx, PlayerInteraction, Victim, RigidBody, Collider) {
            let keyset = ctx.select::<PlayerInteraction>(id).clone().keyset;

            let right_pressed = ctx.key_pressed(keyset.right());
            let left_pressed = ctx.key_pressed(keyset.left());
            let down_pressed = ctx.key_pressed(keyset.down());

            let up_pressed = ctx.key_just_pressed(keyset.up());

            let collider = ctx.select::<Collider>(id).clone();
            let victim = ctx.select::<Victim>(id).clone();
            let player_movement = ctx.select::<PlayerInteraction>(id).clone();
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
            {
                let player_movement = ctx.select::<PlayerInteraction>(id);
                player_movement.jump_state = JumpState::OnGround;
            }

            if up_pressed && player_movement.can_jump() {
                let body = ctx.select::<RigidBody>(id);
                body.vel.1 = -800.0;
                let player_movement = ctx.select::<PlayerInteraction>(id);
                player_movement.jump_state = player_movement.jump_state.next();
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub enum JumpState {
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
}
