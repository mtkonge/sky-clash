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

impl Diagonal {
    pub fn common(&self, other: &Self) -> Option<Direction> {
        use Diagonal::*;
        use Direction::*;
        let direction = match (self, other) {
            (TopLeft, TopRight) => Top,
            (TopLeft, BottomLeft) => Left,
            (TopRight, TopLeft) => Top,
            (TopRight, BottomRight) => Right,
            (BottomRight, TopRight) => Right,
            (BottomRight, BottomLeft) => Bottom,
            (BottomLeft, TopLeft) => Left,
            (BottomLeft, BottomRight) => Bottom,
            _ => return None,
        };
        Some(direction)
    }

    pub fn clockwise(&self) -> (Direction, Direction) {
        use Diagonal::*;
        use Direction::*;
        match self {
            TopLeft => (Left, Top),
            TopRight => (Top, Right),
            BottomRight => (Right, Bottom),
            BottomLeft => (Bottom, Left),
        }
    }
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
        match (self.y >= 0.0, self.x >= 0.0) {
            (false, true) => TopRight,
            (true, true) => BottomRight,
            (false, false) => TopLeft,
            (true, false) => BottomLeft,
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

fn intersect_but_one_of_the_xs_is_regarded(
    p: V2,
    delta_pos: V2,
    p0: V2,
    p1: V2,
) -> Option<(V2, f64)> {
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
                return Some((V2::new(p.x, b1), t));
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
    unreachable!()
}

fn vector_is_parallel(v0: V2, v1: V2) -> bool {
    if v0.x == 0.0 || v1.x == 0.0 {
        return v0.x == v1.x;
    } else if v0.y == 0.0 || v1.y == 0.0 {
        return v0.y == v1.y;
    }
    let kx = v0.x / v1.x;
    let ky = v0.y / v1.y;
    kx == ky
}

fn intersect(p: V2, delta_pos: V2, p0: V2, p1: V2) -> Option<(V2, f64)> {
    let l1 = p1 - p0;
    if delta_pos.x == 0.0 || l1.x == 0.0 {
        return intersect_but_one_of_the_xs_is_regarded(p, delta_pos, p0, p1);
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
    //   x * (ap - a1) = b1 - bp
    //               x = (b1 - bp) / (ap - a1)
    let x = (b1 - bp) / (ap - a1);
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
    //            px = p + t * dp
    //        px - p = t * dp
    // (px - p) / dp = t
    //             t = (px - p) / dp
    let s = (x - p.x) / delta_pos.x;
    if s >= 0.0 {
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
) -> Option<(V2, Direction, f64)> {
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

    let (da, db) = diag.clockwise();

    for (oca, ocb, dir) in [
        (other_c0, other_c1, da.clone()),
        (other_c1, other_c2, db.clone()),
    ] {
        for c in [c0, c1, c2] {
            if let Some((intersection, t)) = intersect(c, delta_pos, oca, ocb) {
                return Some((intersection, dir, t));
            }
        }
    }
    // for (ca, cb, dir) in [(c0, c1, da), (c1, c2, db)] {
    //     for oc in [other_c0, other_c1, other_c2] {
    //         if let Some((_, t)) = intersect(oc, delta_pos, ca, cb) {
    //             return Some((oc, dir, t));
    //         }
    //     }
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
                let body = ctx.entity_component::<RigidBody>(id);
                if let Some((intersection, dir, _t)) = collision {
                    match dir {
                        Direction::Top => {
                            body.pos.1 = intersection.y;
                            body.vel.1 = 0.0;
                        }
                        Direction::Bottom => {
                            body.pos.1 = intersection.y - body.rect.1;
                            body.vel.1 = 0.0;
                        }
                        Direction::Right => {
                            body.pos.0 = intersection.x - body.rect.0;
                            body.vel.0 = 0.0;
                        }
                        Direction::Left => {
                            body.pos.0 = intersection.x;
                            body.vel.0 = 0.0;
                        }
                    }
                }
                // println!(
                //     "{} {} {} {collision:?}",
                //     body.pos.1 + body.rect.1,
                //     other_body.pos.1,
                //     body.pos.1 + body.rect.1 >= other_body.pos.1
                // );
            }
        }
        Ok(())
    }
}
