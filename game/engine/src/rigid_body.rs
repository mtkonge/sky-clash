use super::{Context, Error, System};
use crate::{query, rigid_body, Component};

#[derive(Component, Clone, Debug)]
pub struct RigidBody {
    pub pos: (f64, f64),
    pub vel: (f64, f64),
    pub size: (f64, f64),
    pub gravity: bool,
    pub drag: bool,
}

impl RigidBody {
    pub fn new() -> Self {
        Self {
            pos: (0.0, 0.0),
            vel: (0.0, 0.0),
            size: (0.0, 0.0),
            gravity: false,
            drag: false,
        }
    }

    pub fn with_pos(self, pos: (f64, f64)) -> Self {
        Self { pos, ..self }
    }

    pub fn with_vel(self, vel: (f64, f64)) -> Self {
        Self { vel, ..self }
    }

    pub fn with_size(self, size: (f64, f64)) -> Self {
        Self { size, ..self }
    }

    pub fn with_gravity(self) -> Self {
        Self {
            gravity: true,
            ..self
        }
    }

    pub fn with_drag(self) -> Self {
        Self { drag: true, ..self }
    }
}

pub struct VelocitySystem(pub u64);
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

pub struct GravitySystem(pub u64);
impl System for GravitySystem {
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        for id in query!(ctx, RigidBody) {
            let body = ctx.select::<RigidBody>(id);
            if !body.gravity {
                continue;
            }
            body.vel.1 = if body.vel.1 < 400.0 {
                body.vel.1 + 1600.0 * delta
            } else {
                body.vel.1
            };
        }
        Ok(())
    }
}

pub struct DragSystem(pub u64);
impl System for DragSystem {
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        for id in query!(ctx, RigidBody) {
            let body = ctx.select::<RigidBody>(id);
            if !body.drag {
                continue;
            }
            if body.vel.0 == 0.0 {
                continue;
            }
            let eq = body.vel.0.abs().powf(1.25) * delta * 0.1 + 5.0;
            if body.vel.0 > 10.0 {
                body.vel.0 -= eq;
                if body.vel.0 < 0.0 {
                    body.vel.0 = 0.0
                }
            } else if body.vel.0 < (-10.0) {
                body.vel.0 += eq;
                if body.vel.0 > 0.0 {
                    body.vel.0 = 0.0
                }
            } else {
                body.vel.0 = 0.0
            }
        }
        Ok(())
    }
}
