use super::{Context, Error, System};
use crate::{query, rigid_body, Component, V2};

#[derive(Component, Clone, Debug)]
pub struct RigidBody {
    pub pos: V2,
    pub vel: V2,
    pub size: V2,
    pub gravity: bool,
    pub drag: bool,
}

impl RigidBody {
    pub fn new() -> Self {
        Self {
            pos: V2::new(0.0, 0.0),
            vel: V2::new(0.0, 0.0),
            size: V2::new(0.0, 0.0),
            gravity: false,
            drag: false,
        }
    }

    pub fn with_pos(self, pos: V2) -> Self {
        Self { pos, ..self }
    }

    pub fn with_vel(self, vel: V2) -> Self {
        Self { vel, ..self }
    }

    pub fn with_size(self, size: V2) -> Self {
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
            body.pos += body.vel.extend(delta);
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
            body.vel.y = if body.vel.y < 400.0 {
                body.vel.y + 1600.0 * delta
            } else {
                body.vel.y
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
            if body.vel.x == 0.0 {
                continue;
            }
            let eq = body.vel.x.abs().powf(1.25) * delta * 0.1 + 5.0;
            if body.vel.x > 10.0 {
                body.vel.x -= eq;
                if body.vel.x < 0.0 {
                    body.vel.x = 0.0
                }
            } else if body.vel.x < (-10.0) {
                body.vel.x += eq;
                if body.vel.x > 0.0 {
                    body.vel.x = 0.0
                }
            } else {
                body.vel.x = 0.0
            }
        }
        Ok(())
    }
}
