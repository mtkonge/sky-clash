use crate::{
    engine::{self, Component, System},
    query, RigidBody,
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct V2 {
    pub x: f64,
    pub y: f64,
}

impl V2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn extend(&self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }

    pub fn div_comps(&self, rhs: f64) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }

    pub fn min_comp(&self) -> f64 {
        std::cmp::min_by(self.x, self.y, f64::total_cmp)
    }

    pub fn max_comp(&self) -> f64 {
        std::cmp::max_by(self.x, self.y, f64::total_cmp)
    }

    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn reverse(&self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl std::ops::Add for V2 {
    type Output = V2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for V2 {
    type Output = V2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<(f64, f64)> for V2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

fn point_vec_2_point_line_intersect(p: V2, dp: V2, c0: V2, c1: V2) -> Option<(V2, f64)> {
    if dp.len() == 0.0 {
        // no movement, no collision
        return None;
    }
    let edge = c1 - c0;
    let (x, y) = if dp.x == 0.0 && edge.x == 0.0 {
        // parallel, do nothing
        return None;
    } else if dp.x == 0.0 {
        let x = p.x;
        let ae = edge.y / edge.x;
        let be = c0.y - ae * c0.x;
        let y = ae * x + be;
        (x, y)
    } else if edge.x == 0.0 {
        let y = edge.y;
        let ap = dp.y / dp.x;
        let bp = p.y - ap * p.x;
        let x = (y - bp) / ap;
        (x, y)
    } else {
        let ap = dp.y / dp.x;
        let ae = edge.y / edge.x;
        if ap == ae {
            // parallel: either none or continous intersection
            return None;
        }
        let bp = p.y - ap * p.x;
        let be = c0.y - ae * c0.x;
        let x = (be - bp) / (ap - ae);
        let y = ap * x + bp;
        (x, y)
    };
    let t = if c1.x == c0.x {
        (y - c1.y) / (c1.y - c0.y)
    } else {
        (x - c1.x) / (c1.x - c0.x)
    };
    if !(0.0..1.0).contains(&t) {
        // outside corners
        return None;
    }
    let s = if dp.x == 0.0 {
        (y - p.y) / dp.y
    } else {
        (x - p.x) / dp.x
    };
    if s >= 0.0 {
        // out of range
        return None;
    }
    Some((V2::new(x, y), t))
}

#[derive(Component, Clone, Default)]
pub struct ResolvingBoxCollider {
    pub resolve: bool,
}

pub struct ResolvingBoxCollisionSystem;
impl System for ResolvingBoxCollisionSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody, ResolvingBoxCollider) {
            let collider = ctx.entity_component::<ResolvingBoxCollider>(id).clone();
            if !collider.resolve {
                continue;
            }
            let body = ctx.entity_component::<RigidBody>(id).clone();
            for other_id in query!(ctx, RigidBody, ResolvingBoxCollider) {
                if id == other_id {
                    continue;
                }
                let other_body = ctx.entity_component::<RigidBody>(other_id).clone();
                //
                //let body = ctx.entity_component::<RigidBody>(id);
            }
        }
        Ok(())
    }
}
