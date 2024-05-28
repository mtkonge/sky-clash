use std::rc::Rc;

use engine::rigid_body::RigidBody;
use engine::{query, Collider, Component, V2};
use engine::{Context, Error, System};

use crate::player::Player;
use crate::player_interaction::{DodgeState, PlayerInteraction};
use crate::sprite_renderer::Sprite;
use crate::timer::Timer;

#[derive(Default, Clone)]
pub enum HurtDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Default, Clone)]
pub struct Hitbox {
    pub size: V2,
    pub offset: V2,
}

pub struct Outcome {
    pub damage: f64,
    pub delta_vel: V2,
    pub stun_time: Option<f64>,
}

pub trait HurtboxProfile {
    fn outcome(
        &self,
        victim: &Player,
        attacker: Option<&Player>,
        hurtbox_body: &RigidBody,
    ) -> Outcome;
}

#[derive(Component, Clone)]
pub struct Hurtbox {
    pub owner: Option<engine::Id>,
    pub timer: Timer,
    pub textures: Vec<engine::Texture>,
    pub profile: Rc<dyn HurtboxProfile>,
}

#[derive(Component, Default, Clone)]
pub struct Victim {
    pub hurt_by: Vec<engine::Id>,
    pub stunned: Option<f64>,
}

fn rects_collide(pos_a: V2, size_a: V2, pos_b: V2, size_b: V2) -> bool {
    pos_a.x < pos_b.x + size_b.x
        && pos_a.x + size_a.x > pos_b.x
        && pos_a.y < pos_b.y + size_b.y
        && pos_a.y + size_a.y > pos_b.y
}

pub struct HurtboxSystem(pub u64);
impl System for HurtboxSystem {
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        for id in query!(ctx, Victim) {
            let victim = ctx.select::<Victim>(id);
            self.update_victim_stun_timer(victim, delta);
        }
        self.despawn_expired_hurtboxes(ctx, delta);
        for hurtbox_id in query!(ctx, Hurtbox, RigidBody).clone() {
            let hurtbox_body = ctx.select::<RigidBody>(hurtbox_id).clone();
            let hurtbox = ctx.select::<Hurtbox>(hurtbox_id).clone();
            for victim_id in query!(
                ctx,
                PlayerInteraction,
                RigidBody,
                Collider,
                Player,
                Victim,
                Hitbox
            ) {
                if hurtbox.owner.is_some_and(|owner| owner == victim_id) {
                    continue;
                };
                let victim = ctx.select::<Victim>(victim_id);
                if victim.hurt_by.iter().any(|i_id| *i_id == hurtbox_id) {
                    continue;
                }

                let hitbox = ctx.select::<Hitbox>(victim_id).clone();
                let victim_body = ctx.select::<RigidBody>(victim_id).clone();

                if !rects_collide(
                    hurtbox_body.pos,
                    hurtbox_body.size,
                    victim_body.pos + hitbox.offset,
                    hitbox.size,
                ) {
                    continue;
                };

                let dodge_state = ctx
                    .select::<PlayerInteraction>(victim_id)
                    .clone()
                    .dodge_state;

                if matches!(dodge_state, DodgeState::Dodging(_)) {
                    continue;
                }

                self.hurt_victim(hurtbox_id, &hurtbox, ctx, victim_id, &hurtbox_body);
            }
        }
        for id in query!(ctx, Hurtbox, Sprite).clone() {
            let hurtbox = ctx.select::<Hurtbox>(id).clone();
            let sprite = ctx.select::<Sprite>(id);
            self.draw_hurtbox_animation(hurtbox, sprite);
        }
        Ok(())
    }
}

impl HurtboxSystem {
    fn hurt_victim(
        &self,
        hurtbox_id: u64,
        hurtbox: &Hurtbox,
        ctx: &mut Context,
        victim_id: u64,
        hurtbox_body: &RigidBody,
    ) {
        let attacker = hurtbox.owner.map(|id| ctx.select::<Player>(id).clone());

        let attacker_strength = attacker
            .as_ref()
            .map(|a| a.hero.strength_points)
            .unwrap_or(0);

        let victim = ctx.select::<Player>(victim_id);
        let victim_defence = victim.hero.defence_points;

        let Outcome {
            damage,
            delta_vel,
            stun_time,
        } = hurtbox
            .profile
            .outcome(victim, attacker.as_ref(), hurtbox_body);

        let max_points = 24.0;
        let damage_multiplier = 1.0 + attacker_strength as f64 / (max_points * 2.0)
            - (victim_defence as f64 + 1.0) / (max_points * 2.0);
        let damage = damage * damage_multiplier;
        let victim = ctx.select::<Victim>(victim_id);
        victim.hurt_by.push(hurtbox_id);
        victim.stunned = stun_time;

        let victim_body = ctx.select::<RigidBody>(victim_id);

        victim_body.vel += delta_vel;

        let player = ctx.select::<Player>(victim_id);

        player.damage_taken += damage;
    }

    fn despawn_expired_hurtboxes(&self, ctx: &mut Context, delta: f64) {
        for hurtbox_id in query!(ctx, Hurtbox) {
            let hurtbox = ctx.select::<Hurtbox>(hurtbox_id);
            hurtbox.timer.update(delta);
            if hurtbox.timer.done() {
                ctx.despawn(hurtbox_id);
                continue;
            }
        }
    }

    fn draw_hurtbox_animation(&self, hurtbox: Hurtbox, sprite: &mut Sprite) {
        let texture = hurtbox.textures[std::cmp::min(
            ((hurtbox.timer.time_passed() / hurtbox.timer.duration())
                * hurtbox.textures.len() as f64)
                .floor() as usize,
            hurtbox.textures.len(),
        )];
        sprite.texture = texture;
    }

    fn update_victim_stun_timer(&self, victim: &mut Victim, delta: f64) {
        if let Some(time) = &mut victim.stunned {
            *time -= delta;
            if *time <= 0.0 {
                victim.stunned = None;
            }
        }
    }
}
