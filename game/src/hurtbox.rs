use engine::rigid_body::RigidBody;
use engine::{query, Collider, Component};
use engine::{Context, Error, System};

#[derive(Default, Clone)]
pub enum HurtDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Default, Clone)]
pub struct Hurtbox {
    pub owner: Option<engine::Id>,
    pub power: f64,
    pub direction: HurtDirection,
    pub duration: f64,
    pub duration_passed: f64,
    pub stun_time: Option<f64>,
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

#[derive(Clone, Component)]
pub struct Player {
    pub kind: PlayerKind,
    pub hero: shared::Hero,
    pub knockback_modifier: f64,
    pub lives: i8,
}

#[derive(Clone)]
pub enum PlayerKind {
    Left,
    Right,
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
            let rigid_body = ctx.select::<RigidBody>(id).clone();
            let hurtbox = ctx.select::<Hurtbox>(id);
            hurtbox.duration_passed += delta;
            let hurtbox = ctx.select::<Hurtbox>(id).clone();
            if hurtbox.duration <= hurtbox.duration_passed {
                ctx.despawn(id);
                continue;
            }
            for victim_id in query!(ctx, RigidBody, Collider, Player, Victim) {
                if hurtbox.owner.is_some_and(|owner| owner == victim_id) {
                    continue;
                };
                let victim = ctx.select::<Victim>(victim_id);
                if victim.hurt_by.iter().any(|i_id| *i_id == id) {
                    continue;
                }
                victim.hurt_by.push(id);
                victim.stunned = hurtbox.stun_time;

                let match_hero = ctx.select::<Player>(victim_id);

                let knockback_modifier = match_hero.knockback_modifier + 1.0;
                let body = ctx.select::<RigidBody>(victim_id);
                if !rects_collide(rigid_body.pos, rigid_body.rect, body.pos, body.rect) {
                    continue;
                };

                let velocity = hurtbox.power * knockback_modifier.powi(2) * 0.1;

                match hurtbox.direction {
                    HurtDirection::Up => body.vel.1 -= velocity,
                    HurtDirection::Down => body.vel.1 += velocity,
                    HurtDirection::Left => body.vel.0 -= velocity,
                    HurtDirection::Right => body.vel.0 += velocity,
                }
                let match_hero = ctx.select::<Player>(victim_id);

                match_hero.knockback_modifier += hurtbox.power / 1000.0;
            }
        }
        Ok(())
    }
}
