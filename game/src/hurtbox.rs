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
pub struct MatchHero {
    pub kind: HeroKind,
    pub hero: shared::Hero,
    pub knockback_modifier: f64,
    pub lives: i8,
}

#[derive(Clone)]
pub enum HeroKind {
    Hero1,
    Hero2,
}

pub struct HurtboxSystem(pub u64);
impl System for HurtboxSystem {
    fn on_update(&self, ctx: &mut Context, _delta: f64) -> Result<(), Error> {
        for id in query!(ctx, Hurtbox, RigidBody).clone() {
            let hurtbox = ctx.select::<Hurtbox>(id).clone();
            let rigid_body = ctx.select::<RigidBody>(id).clone();
            for victim_id in query!(ctx, RigidBody, Collider, MatchHero) {
                if hurtbox.owner.is_some_and(|owner| owner == victim_id) {
                    continue;
                };
                let knockback_modifier = ctx.select::<MatchHero>(victim_id).knockback_modifier;
                let victim = ctx.select::<RigidBody>(victim_id);
                if !rects_collide(rigid_body.pos, rigid_body.rect, victim.pos, victim.rect) {
                    continue;
                };
                let velocity = hurtbox.power * knockback_modifier;
                match hurtbox.direction {
                    HurtDirection::Up => victim.vel.1 -= velocity,
                    HurtDirection::Down => victim.vel.1 += velocity,
                    HurtDirection::Left => victim.vel.0 -= velocity,
                    HurtDirection::Right => victim.vel.0 += velocity,
                }
            }
        }
        Ok(())
    }
}
