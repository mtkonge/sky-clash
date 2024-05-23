use engine::{query, rigid_body::RigidBody, spawn, Collider, Component, System, V2};

use crate::{
    hurtbox::{HurtDirection, Hurtbox, HurtboxProfile, Outcome, Victim},
    keyset::Keyset,
    sprite_renderer::Sprite,
    timer::Timer,
};

enum AttackKind {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub enum DodgeState {
    Dodging(Timer),
    Cooldown(Timer),
    Ready,
}

impl DodgeState {
    pub fn update(&mut self, delta: f64) {
        match self {
            DodgeState::Dodging(timer) => {
                timer.update(delta);
                if timer.done() {
                    *self = DodgeState::Cooldown(Timer::new(2.0));
                }
            }
            DodgeState::Cooldown(timer) => {
                timer.update(delta);
                if timer.done() {
                    *self = DodgeState::Ready
                }
            }
            DodgeState::Ready => (),
        }
    }
}

#[derive(Component, Clone)]
pub struct PlayerInteraction {
    pub keyset: Keyset,
    pub attack_cooldown: f64,
    pub jump_state: JumpState,
    pub dodge_state: DodgeState,
}

impl PlayerInteraction {
    pub fn new(keyset: Keyset, attack_cooldown: f64) -> Self {
        Self {
            keyset,
            attack_cooldown,
            jump_state: JumpState::DoubleJumped,
            dodge_state: DodgeState::Ready,
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
        for id in query!(ctx, PlayerInteraction, Victim, RigidBody, Collider) {
            self.update_player_attack(ctx, delta, id)?;
            self.update_player_movement(ctx, delta, id)?;
            self.update_dodge(ctx, delta, id)?;
        }
        Ok(())
    }
}

impl PlayerInteractionSystem {
    fn spawn_attack(
        &self,
        ctx: &mut engine::Context,
        attack_kind: AttackKind,
        id: u64,
        body: &RigidBody,
    ) {
        let attack_size = self.attack_size(&attack_kind);
        let pos = self.attack_pos(&attack_kind, body, attack_size);
        let vel = self.attack_vel(&attack_kind, body.vel);
        let textures = self.attack_textures(ctx, &attack_kind);
        let profile = self.attack_profile(&attack_kind).into();
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
                owner: Some(id),
                timer: Timer::new(0.3),
                textures,
                profile,
            }
        );
    }

    fn attack_size(&self, attack_kind: &AttackKind) -> V2 {
        match attack_kind {
            AttackKind::Up => V2::new(128.0, 64.0),
            AttackKind::Down => V2::new(128.0 * 2.0, 32.0),
            AttackKind::Left => V2::new(64.0, 128.0),
            AttackKind::Right => V2::new(64.0, 128.0),
        }
    }

    fn attack_pos(&self, attack_kind: &AttackKind, body: &RigidBody, attack_size: V2) -> V2 {
        match attack_kind {
            AttackKind::Up => V2::new(
                body.pos.x + (body.size.x - attack_size.x) / 2.0,
                body.pos.y - attack_size.y,
            ),
            AttackKind::Down => V2::new(
                body.pos.x + (body.size.x - attack_size.x) / 2.0,
                body.pos.y + body.size.y - attack_size.y,
            ),
            AttackKind::Left => V2::new(
                body.pos.x - attack_size.x,
                body.pos.y + (body.size.y - attack_size.y) / 2.0,
            ),
            AttackKind::Right => V2::new(
                body.pos.x + body.size.x,
                body.pos.y + (body.size.y - attack_size.y) / 2.0,
            ),
        }
    }

    fn attack_vel(&self, attack_kind: &AttackKind, vel: V2) -> V2 {
        match attack_kind {
            AttackKind::Up => V2::new(0.0, 0.0),
            AttackKind::Down => V2::new(0.0, 0.0),
            AttackKind::Left => vel.div_comps(2.0),
            AttackKind::Right => vel.div_comps(2.0),
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
        id: u64,
    ) -> Result<(), engine::Error> {
        let player_attack = ctx.select::<PlayerInteraction>(id).clone();
        let keyset = player_attack.keyset;
        let right_pressed = ctx.key_pressed(keyset.right());
        let left_pressed = ctx.key_pressed(keyset.left());
        let down_pressed = ctx.key_pressed(keyset.down());
        let light_attack_pressed = ctx.key_just_pressed(keyset.light_attack());
        let victim = ctx.select::<Victim>(id).clone();
        let body = ctx.select::<RigidBody>(id).clone();

        if matches!(player_attack.dodge_state, DodgeState::Dodging(_)) {
            return Ok(());
        }

        if victim.stunned.is_some() {
            for hurtbox_id in query!(ctx, Hurtbox, RigidBody) {
                let hurtbox = ctx.select::<Hurtbox>(hurtbox_id);
                if hurtbox.owner.is_some_and(|owner| owner == id) {
                    ctx.despawn(hurtbox_id)
                };
            }
            return Ok(());
        }

        if player_attack.attack_cooldown >= 0.0 {
            let player_attack = ctx.select::<PlayerInteraction>(id);
            player_attack.attack_cooldown -= delta;
            return Ok(());
        }

        if !light_attack_pressed {
            return Ok(());
        }

        if down_pressed {
            self.spawn_attack(ctx, AttackKind::Down, id, &body)
        } else if left_pressed && !right_pressed {
            self.spawn_attack(ctx, AttackKind::Left, id, &body)
        } else if right_pressed && !left_pressed {
            self.spawn_attack(ctx, AttackKind::Right, id, &body)
        } else {
            self.spawn_attack(ctx, AttackKind::Up, id, &body)
        }
        let player_attack = ctx.select::<PlayerInteraction>(id);
        player_attack.attack_cooldown = 0.5;

        Ok(())
    }

    fn update_player_movement(
        &self,
        ctx: &mut engine::Context,
        delta: f64,
        id: u64,
    ) -> Result<(), engine::Error> {
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
            return Ok(());
        }

        if right_pressed && !left_pressed && body.vel.x < 400.0 {
            body.vel.x += 400.0 * delta * 8.0
        } else if left_pressed && !right_pressed && body.vel.x > (-400.0) {
            body.vel.x -= 400.0 * delta * 8.0
        }

        if down_pressed && body.vel.y < 800.0 {
            body.vel.y += 3200.0 * delta
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
            body.vel.y = -800.0;
            let player_movement = ctx.select::<PlayerInteraction>(id);
            player_movement.jump_state = player_movement.jump_state.next();
        }
        Ok(())
    }

    fn update_dodge(
        &self,
        ctx: &mut engine::Context,
        delta: f64,
        id: u64,
    ) -> Result<(), engine::Error> {
        let player_interaction = ctx.select::<PlayerInteraction>(id);
        let keyset = player_interaction.keyset.clone();
        let dodge_state = &mut player_interaction.dodge_state;

        dodge_state.update(delta);

        match dodge_state {
            DodgeState::Dodging(_) => return Ok(()),
            DodgeState::Cooldown(_) => {
                let sprite = ctx.select::<Sprite>(id);
                sprite.set_opacity(1.0);
                return Ok(());
            }
            DodgeState::Ready => (),
        }

        let dodge_pressed = ctx.key_just_pressed(keyset.dodge());

        let victim = ctx.select::<Victim>(id);

        if !dodge_pressed || victim.stunned.is_some() {
            return Ok(());
        }

        let player_interaction = ctx.select::<PlayerInteraction>(id);
        let dodge_state = &mut player_interaction.dodge_state;
        *dodge_state = DodgeState::Dodging(Timer::new(1.0));

        let sprite = ctx.select::<Sprite>(id);
        sprite.set_opacity(0.5);

        Ok(())
    }

    fn attack_profile(&self, attack_kind: &AttackKind) -> Box<dyn HurtboxProfile> {
        match attack_kind {
            AttackKind::Up => Box::new(UpAttackProfile),
            AttackKind::Down => Box::new(DownAttackProfile),
            AttackKind::Left => Box::new(SideAttackProfile {
                direction: HurtDirection::Left,
            }),
            AttackKind::Right => Box::new(SideAttackProfile {
                direction: HurtDirection::Right,
            }),
        }
    }
}

struct SideAttackProfile {
    direction: HurtDirection,
}

impl HurtboxProfile for SideAttackProfile {
    fn outcome(&self, player: &crate::player::Player, hurtbox_body: &RigidBody) -> Outcome {
        let power = 20.0;
        let knockback_modifier = player.damage_taken / 75.0 + 1.0;

        let hurtbox_vel = (hurtbox_body.vel.x.powi(2) + hurtbox_body.vel.y.powi(2)).sqrt();
        let velocity = hurtbox_vel
            + power * knockback_modifier.powi(2) * 0.8
            + power * 10.0
            + knockback_modifier * 5.0;

        let delta_vel = match self.direction {
            HurtDirection::Left => V2::new(-velocity, 0.0),
            HurtDirection::Right => V2::new(velocity, 0.0),
            _ => unreachable!(),
        };

        Outcome {
            damage: 10.0,
            delta_vel,
            stun_time: Some(0.3),
        }
    }
}

struct UpAttackProfile;

impl HurtboxProfile for UpAttackProfile {
    fn outcome(&self, player: &crate::player::Player, hurtbox_body: &RigidBody) -> Outcome {
        let power = 50.0;
        let knockback_modifier = player.damage_taken / 75.0 + 1.0;

        let hurtbox_vel = (hurtbox_body.vel.x.powi(2) + hurtbox_body.vel.y.powi(2)).sqrt();
        let velocity = hurtbox_vel
            + power * knockback_modifier.powi(2) * 0.8
            + power * 10.0
            + knockback_modifier * 5.0;

        let delta_vel = V2::new(0.0, -velocity);

        Outcome {
            damage: 20.0,
            delta_vel,
            stun_time: Some(0.3),
        }
    }
}

struct DownAttackProfile;
impl HurtboxProfile for DownAttackProfile {
    fn outcome(&self, _player: &crate::player::Player, hurtbox_body: &RigidBody) -> Outcome {
        let power = 55.0;
        let knockback_modifier: f64 = 2.0;

        let hurtbox_vel = (hurtbox_body.vel.x.powi(2) + hurtbox_body.vel.y.powi(2)).sqrt();
        let velocity = hurtbox_vel
            + power * knockback_modifier.powi(2) * 0.8
            + power * 10.0
            + knockback_modifier * 5.0;

        let delta_vel = V2::new(0.0, -velocity);

        Outcome {
            damage: 5.0,
            delta_vel,
            stun_time: Some(0.3),
        }
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
