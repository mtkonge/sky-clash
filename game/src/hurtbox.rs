use engine::rigid_body::RigidBody;
use engine::{query, Collider, Component};
use engine::{Context, Error, System};

#[derive(Component, Default, Clone, Debug)]
pub struct Hurtbox {
    pub owner: Option<engine::Id>,
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
    fn on_update(&self, ctx: &mut Context, _delta: f64) -> Result<(), Error> {
        for id in query!(ctx, Hurtbox, RigidBody).clone() {
            let hurtbox_owner = ctx.select::<Hurtbox>(id).owner;
            let hurtbox = ctx.select::<RigidBody>(id).clone();
            for rigid_body_id in query!(ctx, RigidBody, Collider) {
                if hurtbox_owner.is_some_and(|owner| owner == rigid_body_id) {
                    continue;
                };
                let victim = ctx.select::<RigidBody>(rigid_body_id);
                if !rects_collide(hurtbox.pos, hurtbox.rect, victim.pos, victim.rect) {
                    continue;
                };
            }
        }
        Ok(())
    }
}
