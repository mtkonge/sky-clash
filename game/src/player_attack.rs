use engine::{query, rigid_body::RigidBody, spawn, Collider, Component, System};

use crate::{
    hurtbox::{HurtDirection, Hurtbox, Victim},
    keyset::Keyset,
    sprite_renderer::Sprite,
};

#[derive(Component, Clone)]
pub struct PlayerAttack {
    pub keyset: Keyset,
    pub cooldown: f64,
}

impl PlayerAttack {
    pub fn new(keyset: Keyset, cooldown: f64) -> Self {
        Self { keyset, cooldown }
    }
}

pub struct PlayerAttackSystem(pub u64);
impl System for PlayerAttackSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody, Collider, PlayerAttack, Victim) {
            let player_attack = ctx.select::<PlayerAttack>(id).clone();
            let keyset = player_attack.keyset;
            let right_pressed = ctx.key_pressed(keyset.right());
            let left_pressed = ctx.key_pressed(keyset.left());
            let down_pressed = ctx.key_pressed(keyset.down());
            let light_attack_pressed = ctx.key_just_pressed(keyset.light_attack());
            let victim = ctx.select::<Victim>(id).clone();
            let body = ctx.select::<RigidBody>(id).clone();
            if player_attack.cooldown >= 0.0 {
                let player_attack = ctx.select::<PlayerAttack>(id);
                player_attack.cooldown -= delta;
                continue;
            }
            if !light_attack_pressed || victim.stunned.is_some() {
                continue;
            }
            if down_pressed {
                self.spawn_attack(ctx, HurtDirection::Down, id, &body)
            } else if left_pressed && !right_pressed {
                self.spawn_attack(ctx, HurtDirection::Left, id, &body)
            } else if right_pressed && !left_pressed {
                self.spawn_attack(ctx, HurtDirection::Right, id, &body)
            } else {
                self.spawn_attack(ctx, HurtDirection::Up, id, &body)
            }
            let player_attack = ctx.select::<PlayerAttack>(id);
            player_attack.cooldown = 0.5;
        }

        Ok(())
    }
}

impl PlayerAttackSystem {
    fn spawn_attack(
        &self,
        ctx: &mut engine::Context,
        direction: HurtDirection,
        id: u64,
        body: &RigidBody,
    ) {
        let attack_size = self.attack_size(&direction);
        let pos = self.attack_pos(&direction, body, attack_size);
        let vel = self.attack_vel(&direction, body.vel);
        let textures = self.attack_textures(ctx, &direction);
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

    fn attack_size(&self, direction: &HurtDirection) -> (f64, f64) {
        match direction {
            HurtDirection::Up => (128.0, 64.0),
            HurtDirection::Down => (128.0 * 2.0, 32.0),
            HurtDirection::Left => (64.0, 128.0),
            HurtDirection::Right => (64.0, 128.0),
        }
    }

    fn attack_pos(
        &self,
        direction: &HurtDirection,
        body: &RigidBody,
        attack_size: (f64, f64),
    ) -> (f64, f64) {
        match direction {
            HurtDirection::Up => (
                body.pos.0 + (body.size.0 - attack_size.0) / 2.0,
                body.pos.1 - attack_size.1,
            ),
            HurtDirection::Down => (
                body.pos.0 + (body.size.0 - attack_size.0) / 2.0,
                body.pos.1 + body.size.1 - attack_size.1,
            ),
            HurtDirection::Left => (
                body.pos.0 - attack_size.0,
                body.pos.1 + (body.size.1 - attack_size.1) / 2.0,
            ),
            HurtDirection::Right => (
                body.pos.0 + body.size.0,
                body.pos.1 + (body.size.1 - attack_size.1) / 2.0,
            ),
        }
    }

    fn attack_vel(&self, direction: &HurtDirection, vel: (f64, f64)) -> (f64, f64) {
        match direction {
            HurtDirection::Up => (0.0, 0.0),
            HurtDirection::Down => (0.0, 0.0),
            HurtDirection::Left => (vel.0 / 2.0, vel.1 / 2.0),
            HurtDirection::Right => (vel.0 / 2.0, vel.1 / 2.0),
        }
    }

    fn attack_textures(
        &self,
        ctx: &mut engine::Context,
        direction: &HurtDirection,
    ) -> Vec<engine::Texture> {
        match direction {
            HurtDirection::Up => vec![
                "textures/attacks/up_0.png".to_string(),
                "textures/attacks/up_1.png".to_string(),
                "textures/attacks/up_2.png".to_string(),
                "textures/attacks/up_3.png".to_string(),
                "textures/attacks/up_4.png".to_string(),
            ],
            HurtDirection::Down => vec![
                "textures/attacks/down_0.png".to_string(),
                "textures/attacks/down_1.png".to_string(),
                "textures/attacks/down_2.png".to_string(),
                "textures/attacks/down_3.png".to_string(),
                "textures/attacks/down_4.png".to_string(),
                "textures/attacks/down_5.png".to_string(),
                "textures/attacks/down_6.png".to_string(),
                "textures/attacks/down_7.png".to_string(),
            ],
            HurtDirection::Left => vec![
                "textures/attacks/left_0.png".to_string(),
                "textures/attacks/left_1.png".to_string(),
                "textures/attacks/left_2.png".to_string(),
                "textures/attacks/left_3.png".to_string(),
                "textures/attacks/left_4.png".to_string(),
            ],
            HurtDirection::Right => vec![
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
}
