use crate::{
    engine::{self, Component, System},
    query, RigidBody,
};

#[derive(Clone, Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Debug)]
enum Diagonal {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

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

    pub fn diag(&self) -> Diagonal {
        use Diagonal::*;
        match (self.x >= 0.0, self.y >= 0.0) {
            (true, true) => BottomRight,
            (true, false) => TopRight,
            (false, true) => BottomLeft,
            (false, false) => TopLeft,
        }
    }
}

impl From<(f64, f64)> for V2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
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

fn rect_diagonal_corners(pos: V2, delta_pos: V2, rect: V2) -> (Diagonal, V2, V2, V2) {
    use Diagonal::*;
    let diag = delta_pos.diag();
    let (c0, c1, c2) = match diag {
        TopLeft => (pos + V2::new(0.0, rect.y), pos, pos + V2::new(rect.x, 0.0)),
        TopRight => (pos, pos + V2::new(rect.x, 0.0), pos + rect),
        BottomRight => (
            pos + V2::new(rect.x, 0.0),
            pos + rect,
            pos + V2::new(0.0, rect.y),
        ),
        BottomLeft => (pos + rect, pos + V2::new(0.0, rect.y), pos),
    };
    (diag, c0, c1, c2)
}

fn intersect(p: V2, delta_pos: V2, p0: V2, p1: V2) -> Option<(V2, f64)> {
    let l1 = p1 - p0;
    if delta_pos.x == 0.0 && l1.x == 0.0 {
        return None;
    } else if l1.x == 0.0 {
        let ap = delta_pos.y / delta_pos.x;
        let bp = p.y - ap * p.x;
        let t = (bp - p0.y) / l1.y;
        if !(0.0..1.0).contains(&t) {
            return None;
        }
        return Some((V2::new(l1.x, bp), t));
    } else if delta_pos.x == 0.0 {
        let a1 = l1.y / l1.x;
        let b1 = p0.y - a1 * p0.x;
        if l1.y == 0.0 {
            let t = (p.x - p0.x) / (p1.x - p0.x);
            if (p0.x..p1.x).contains(&p.x) {
                return Some((V2::new(l1.x, b1), t));
            } else {
                return None;
            }
        }
        let t = (b1 - p0.y) / l1.y;
        if !(0.0..1.0).contains(&t) {
            return None;
        }
        return Some((V2::new(l1.x, b1), t));
    }
    // y = ax + b
    let ap = delta_pos.y / delta_pos.x;
    let a1 = l1.y / l1.x;
    if ap == a1 {
        return None;
    }

    // b = y - ax
    let bp = p.y - ap * p.x;
    let b1 = p0.y - a1 * p0.x;
    //               y = ap * x + bp
    //               y = a1 * x + b1
    //     ap * x + bp = a1 * x + b1
    // ap * x - a1 * x = + b1 - bp
    //   x * (ap - a1) = bp - b1
    //               x = (bp - b1) / (ap - a1)

    let x = (bp - b1) / (ap - a1);
    let y = ap * x + bp;
    // vec(x, y) = p0 + vec(p0, p1) * t
    // x = p0.x + (p1.x - p0.x) * t
    // x - p0.x = (p1.x - p0.x) * t
    // (x - p0.x) / (p1.x - p0.x) = t
    // t = (x - p0.x) / (p1.x - p0.x)
    let t = if p1.x == p0.x {
        (y - p0.y) / (p1.y - p0.y)
    } else {
        (x - p0.x) / (p1.x - p0.x)
    };
    if !(0.0..1.0).contains(&t) {
        return None;
    }
    Some((V2 { x, y }, t))
}

fn rects_collide(
    pos: V2,
    delta_pos: V2,
    rect: V2,
    other_pos: V2,
    other_delta_pos: V2,
    other_rect: V2,
) -> Option<(V2, Diagonal, f64)> {
    let center = pos + rect.div_comps(2.);
    let radius = rect.div_comps(2.0).max_comp() + delta_pos.len();
    let other_center = other_pos + other_rect.div_comps(2.);
    let other_radius = other_rect.div_comps(2.0).max_comp() + other_delta_pos.len();
    if (center - other_center).len() > radius + other_radius {
        return None;
    }

    let (diag, c0, c1, c2) = rect_diagonal_corners(pos, delta_pos, rect);
    let (other_diag, other_c0, other_c1, other_c2) =
        rect_diagonal_corners(other_pos, other_delta_pos, other_rect);
    println!("d    ({diag:?}, {c1:?})");
    println!("o   ({other_diag:?}, {other_c1:?})");

    // if let Some((intersection, t)) = intersect(c0, delta_pos, other_c0, other_c1) {
    //     return Some((intersection, diag, t));
    // }
    if let Some((intersection, t)) = intersect(c1, delta_pos, other_c0, other_c1) {
        return Some((intersection, diag, t));
    }
    // if let Some((intersection, t)) = intersect(c2, delta_pos, other_c0, other_c1) {
    //     return Some((intersection, diag, t));
    // }
    // if let Some((intersection, t)) = intersect(c0, delta_pos, other_c1, other_c2) {
    //     return Some((intersection, diag, t));
    // }
    // if let Some((intersection, t)) = intersect(c1, delta_pos, other_c1, other_c2) {
    //     return Some((intersection, diag, t));
    // }
    // if let Some((intersection, t)) = intersect(c2, delta_pos, other_c1, other_c2) {
    //     return Some((intersection, diag, t));
    // }

    // if let Some((_, t)) = intersect(other_c0, delta_pos, c0, c1) {
    //     return Some((other_c0, diag, t));
    // }
    // if let Some((_, t)) = intersect(other_c1, delta_pos, c0, c1) {
    //     return Some((other_c1, diag, t));
    // }
    // if let Some((_, t)) = intersect(other_c2, delta_pos, c0, c1) {
    //     return Some((other_c2, diag, t));
    // }
    // if let Some((_, t)) = intersect(other_c0, delta_pos, c1, c2) {
    //     return Some((other_c0, diag, t));
    // }
    // if let Some((_, t)) = intersect(other_c1, delta_pos, c1, c2) {
    //     return Some((other_c1, diag, t));
    // }
    // if let Some((_, t)) = intersect(other_c2, delta_pos, c1, c2) {
    //     return Some((other_c2, diag, t));
    // }
    None
}

#[derive(Component, Clone, Default)]
pub struct Collider {
    pub resolve: bool,
    pub on_ground: bool,
}

pub struct CollisionSystem;
impl System for CollisionSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody, Collider) {
            let collider = ctx.entity_component::<Collider>(id);
            collider.on_ground = false;
            let collider = ctx.entity_component::<Collider>(id).clone();
            if !collider.resolve {
                continue;
            }
            let body = ctx.entity_component::<RigidBody>(id).clone();
            for other_id in query!(ctx, RigidBody, Collider) {
                if id == other_id {
                    continue;
                }
                let other_body = ctx.entity_component::<RigidBody>(other_id).clone();
                let collision = rects_collide(
                    body.pos.into(),
                    V2::from(body.vel).extend(delta),
                    body.rect.into(),
                    other_body.pos.into(),
                    V2::from(body.vel).extend(delta).reverse(),
                    other_body.rect.into(),
                );
                println!(
                    "{} {} {} {collision:?}",
                    body.pos.1 + body.rect.1,
                    other_body.pos.1,
                    body.pos.1 + body.rect.1 >= other_body.pos.1
                );
            }
        }
        Ok(())
    }
}
