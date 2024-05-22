use engine::rigid_body::RigidBody;
use engine::{query, Collider, Component};
use engine::{Context, Error, System};

use crate::player::Player;
use crate::sprite_renderer::Sprite;

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
    pub size: (f64, f64),
    pub offset: (f64, f64),
}

#[derive(Component, Default, Clone)]
pub struct Hurtbox {
    pub owner: Option<engine::Id>,
    pub power: f64,
    pub direction: HurtDirection,
    pub duration: f64,
    pub duration_passed: f64,
    pub stun_time: Option<f64>,
    pub textures: Vec<engine::Texture>,
}

#[derive(Component, Default, Clone)]
pub struct Victim {
    pub hurt_by: Vec<engine::Id>,
    pub stunned: Option<f64>,
}

fn rects_collide(
    pos_a: (f64, f64),
    size_a: (f64, f64),
    pos_b: (f64, f64),
    size_b: (f64, f64),
) -> bool {
    pos_a.0 < pos_b.0 + size_b.0
        && pos_a.0 + size_a.0 > pos_b.0
        && pos_a.1 < pos_b.1 + size_b.1
        && pos_a.1 + size_a.1 > pos_b.1
}

pub struct HurtboxSystem(pub u64);
impl System for HurtboxSystem {
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        for id in query!(ctx, Victim) {
            let victim = ctx.select::<Victim>(id);
            let _ = victim.stunned.as_mut().map(|time| *time -= delta);
            if let Some(time) = victim.stunned {
                if time <= 0.0 {
                    victim.stunned = None;
                }
            }
        }
        for id in query!(ctx, Hurtbox, RigidBody).clone() {
            let hurtbox_body = ctx.select::<RigidBody>(id).clone();
            let hurtbox = ctx.select::<Hurtbox>(id);
            hurtbox.duration_passed += delta;
            let hurtbox = ctx.select::<Hurtbox>(id).clone();
            if hurtbox.duration <= hurtbox.duration_passed {
                ctx.despawn(id);
                continue;
            }
            for victim_id in query!(ctx, RigidBody, Collider, Player, Victim, Hitbox) {
                if hurtbox.owner.is_some_and(|owner| owner == victim_id) {
                    continue;
                };
                let victim = ctx.select::<Victim>(victim_id);
                if victim.hurt_by.iter().any(|i_id| *i_id == id) {
                    continue;
                }
                victim.hurt_by.push(id);
                victim.stunned = hurtbox.stun_time;

                let hitbox = ctx.select::<Hitbox>(victim_id).clone();
                let player = ctx.select::<Player>(victim_id);

                let knockback_modifier = player.knockback_modifier + 1.0;
                let victim_body = ctx.select::<RigidBody>(victim_id);
                if !rects_collide(
                    hurtbox_body.pos,
                    hurtbox_body.size,
                    (
                        victim_body.pos.0 + hitbox.offset.0,
                        victim_body.pos.1 + hitbox.offset.1,
                    ),
                    hitbox.size,
                ) {
                    continue;
                };

                let hurtbox_vel = (hurtbox_body.vel.0.powi(2) + hurtbox_body.vel.1.powi(2)).sqrt();
                let velocity = hurtbox_vel
                    + hurtbox.power * knockback_modifier.powi(2) * 0.8
                    + hurtbox.power * 10.0
                    + knockback_modifier * 5.0;

                match hurtbox.direction {
                    HurtDirection::Up => victim_body.vel.1 -= velocity,
                    HurtDirection::Down => victim_body.vel.1 += velocity,
                    HurtDirection::Left => victim_body.vel.0 -= velocity,
                    HurtDirection::Right => victim_body.vel.0 += velocity,
                }
                let player = ctx.select::<Player>(victim_id);

                player.knockback_modifier += hurtbox.power / 50.0;
            }
        }
        for id in query!(ctx, Hurtbox, Sprite).clone() {
            let hurtbox = ctx.select::<Hurtbox>(id);
            if hurtbox.textures.len() <= 1 {
                continue;
            }
            let texture = hurtbox.textures[std::cmp::min(
                (hurtbox.duration_passed / hurtbox.duration * hurtbox.textures.len() as f64).floor()
                    as usize,
                hurtbox.textures.len(),
            )];
            let sprite = ctx.select::<Sprite>(id);
            sprite.texture = texture;
        }
        Ok(())
    }
}
