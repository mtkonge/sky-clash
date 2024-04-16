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

    pub fn add_x(&self, rhs: f64) -> Self {
        Self::new(self.x + rhs, self.y)
    }

    pub fn add_y(&self, rhs: f64) -> Self {
        Self::new(self.x, self.y + rhs)
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

fn rects_within_reach(pos: V2, delta_pos: V2, rect: V2, other_pos: V2, other_rect: V2) -> bool {
    let radius = rect.div_comps(2.0).len();
    let other_radius = other_rect.div_comps(2.0).len();
    let length_between = (pos - other_pos).len();
    let radii = radius + delta_pos.len() + other_radius;
    radii >= length_between
}

#[test]
fn test_rects_within_reach() {
    assert_eq!(
        rects_within_reach(
            V2::new(0.0, 0.0),
            V2::new(10.0, 0.0),
            V2::new(10.0, 10.0),
            V2::new(15.0, 0.0),
            V2::new(10.0, 10.0)
        ),
        true,
    );
    assert_eq!(
        rects_within_reach(
            V2::new(0.0, 0.0),
            V2::new(10.0, 0.0),
            V2::new(10.0, 10.0),
            V2::new(30.0, 0.0),
            V2::new(10.0, 10.0)
        ),
        false,
    );
}

fn point_vec_2p_line_intersect(p: V2, dp: V2, c0: V2, c1: V2) -> Option<(V2, f64)> {
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
        (y - c0.y) / (c1.y - c0.y)
    } else {
        (x - c0.x) / (c1.x - c0.x)
    };
    if !(0.0..1.0).contains(&t) {
        // outside corners
        return None;
    }
    let s = if dp.x == 0.0 {
        (y - (p.y + dp.y)) / dp.y
    } else {
        (x - (p.x + dp.x)) / dp.x
    };
    if s >= 0.0 {
        // out of range
        return None;
    }
    Some((V2::new(x, y), t))
}

#[test]
fn test_point_vec_2_point_line_intersect() {
    assert_eq!(
        point_vec_2p_line_intersect(
            V2::new(10.0, 10.0),
            V2::new(0.0, 15.0),
            V2::new(0.0, 20.0),
            V2::new(20.0, 20.0)
        ),
        Some((V2::new(10.0, 20.0), 0.5)),
    );
    assert_eq!(
        point_vec_2p_line_intersect(
            V2::new(10.0, 10.0),
            V2::new(0.0, 5.0),
            V2::new(0.0, 20.0),
            V2::new(20.0, 20.0)
        ),
        None,
    );
    assert_eq!(
        point_vec_2p_line_intersect(
            V2::new(30.0, 10.0),
            V2::new(0.0, 15.0),
            V2::new(0.0, 20.0),
            V2::new(20.0, 20.0)
        ),
        None,
    );
}

#[derive(Clone, Copy, Debug)]
enum Boyk {
    Positive,
    Zero,
    Negative,
}

impl From<f64> for Boyk {
    fn from(value: f64) -> Self {
        use Boyk::*;
        if value < 0.0 {
            Positive
        } else if value == 0.0 {
            Zero
        } else {
            Negative
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    None,
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl Direction {
    pub fn reverse(&self) -> Self {
        use Direction::*;
        match self {
            None => None,
            Top => Bottom,
            Right => Left,
            Bottom => Top,
            Left => Right,
            TopLeft => BottomRight,
            TopRight => BottomLeft,
            BottomRight => TopLeft,
            BottomLeft => TopRight,
        }
    }

    pub fn clockwise(&self) -> (Self, Self) {
        use Direction::*;
        match self {
            TopLeft => (Left, Top),
            TopRight => (Top, Right),
            BottomRight => (Right, Bottom),
            BottomLeft => (Bottom, Left),
            _ => unreachable!(),
        }
    }
}

impl From<V2> for Direction {
    fn from(value: V2) -> Self {
        use Boyk::*;
        use Direction::*;
        match (Boyk::from(value.x), Boyk::from(value.y)) {
            (Zero, Zero) => None,
            (Zero, Positive) => Top,
            (Zero, Negative) => Bottom,
            (Positive, Zero) => Left,
            (Negative, Zero) => Right,
            (Positive, Positive) => TopLeft,
            (Positive, Negative) => BottomLeft,
            (Negative, Positive) => TopRight,
            (Negative, Negative) => BottomRight,
        }
    }
}

fn rect_side_corners(pos: V2, rect: V2, dir: Direction) -> (V2, V2) {
    use Direction::*;
    match dir {
        Top => (pos, pos.add_x(rect.x)),
        Right => (pos.add_x(rect.x), pos + rect),
        Bottom => (pos + rect, pos.add_y(rect.y)),
        Left => (pos.add_y(rect.y), pos),
        _ => unreachable!(),
    }
}

fn rect_diagonal_corners(pos: V2, rect: V2, dir: Direction) -> (V2, V2, V2) {
    use Direction::*;
    match dir {
        TopLeft => (pos.add_y(rect.y), pos, pos.add_x(rect.x)),
        TopRight => (pos, pos.add_x(rect.x), pos + rect),
        BottomRight => (pos.add_x(rect.x), pos + rect, pos.add_y(rect.y)),
        BottomLeft => (pos + rect, pos.add_y(rect.y), pos),
        _ => unreachable!(),
    }
}

fn resolve_collision(body: &mut RigidBody, int: V2, rect: V2, dir: Direction) {
    use Direction::*;
    match dir {
        Top => {
            body.pos.1 = int.y + 1.0;
            body.vel.1 = 0.0;
        }
        Bottom => {
            body.pos.1 = int.y - rect.y - 1.0;
            body.vel.1 = 0.0;
        }
        Right => {
            body.pos.0 = int.x + 1.0;
            body.vel.0 = 0.0;
        }
        Left => {
            body.pos.0 = int.x - rect.x - 1.0;
            body.vel.0 = 0.0;
        }
        _ => unreachable!(),
    }
}

#[derive(Component, Clone, Default)]
pub struct Collider {
    pub resolve: bool,
}

pub struct CollisionSystem;
impl System for CollisionSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody, Collider) {
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

                let pos = V2::from(body.pos);
                let rect = V2::from(body.rect);
                let other_pos = V2::from(other_body.pos);
                let other_rect = V2::from(other_body.rect);
                let dp = V2::from(body.vel).extend(delta);

                if !rects_within_reach(pos, dp, rect, other_pos, other_rect) {
                    continue;
                }

                let body = ctx.entity_component::<RigidBody>(id);
                match Direction::from(dp) {
                    Direction::None => {}
                    dir @ (Direction::Top
                    | Direction::Right
                    | Direction::Bottom
                    | Direction::Left) => {
                        let (p0, p1) = rect_side_corners(pos, rect, dir);
                        let (c0, c1) = rect_side_corners(other_pos, other_rect, dir.reverse());
                        for p in [p0, p1] {
                            if let Some((int, _t)) = point_vec_2p_line_intersect(p, dp, c0, c1) {
                                resolve_collision(body, int, rect, dir);
                            }
                        }
                        for p in [c0, c1] {
                            if let Some((_int, _t)) = point_vec_2p_line_intersect(p, dp, p0, p1) {
                                resolve_collision(body, p, rect, dir);
                            }
                        }
                    }
                    dir @ (Direction::TopLeft
                    | Direction::TopRight
                    | Direction::BottomRight
                    | Direction::BottomLeft) => {
                        let (p0, p1, p2) = rect_diagonal_corners(pos, rect, dir);
                        let (c0, c1, c2) =
                            rect_diagonal_corners(other_pos, other_rect, dir.reverse());
                        let (d0, d1) = dir.clockwise();
                        for p in [p0, p1, p2] {
                            for (c0, c1, dir) in [(c0, c1, d0), (c1, c2, d1)] {
                                if let Some((int, _t)) = point_vec_2p_line_intersect(p, dp, c0, c1)
                                {
                                    resolve_collision(body, int, rect, dir);
                                }
                            }
                        }
                        for p in [c0, c1, c2] {
                            for (c0, c1, dir) in [(p0, p1, d0), (p1, p2, d1)] {
                                if let Some((_int, _t)) = point_vec_2p_line_intersect(p, dp, c0, c1)
                                {
                                    resolve_collision(body, p, rect, dir);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
