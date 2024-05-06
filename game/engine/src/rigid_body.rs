use super::{Context, Error, System};
use crate::{query, Component};

#[derive(Component, Default, Clone, Debug)]
pub struct RigidBody {
    pub pos: (f64, f64),
    pub vel: (f64, f64),
    pub rect: (f64, f64),
    pub gravity: bool,
}

pub struct VelocitySystem;
impl System for VelocitySystem {
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        for id in query!(ctx, RigidBody) {
            let body = ctx.select::<RigidBody>(id);
            body.pos.0 += body.vel.0 * delta;
            body.pos.1 += body.vel.1 * delta;
        }
        Ok(())
    }
}

pub struct GravitySystem;
impl System for GravitySystem {
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        for id in query!(ctx, RigidBody) {
            let body = ctx.select::<RigidBody>(id);
            if !body.gravity {
                continue;
            }
            body.vel.1 = if body.vel.1 < 400.0 {
                body.vel.1 + 800.0 * delta
            } else {
                body.vel.1
            };
        }
        Ok(())
    }
}
