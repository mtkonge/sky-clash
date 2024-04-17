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
    let radii = radius + delta_pos.len() + other_radius;
    let length_between = (pos - other_pos).len() - radius - other_radius;
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
            V2::new(40.0, 0.0),
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
        let x = c0.x;
        let ae = dp.y / dp.x;
        let be = p.y - ae * p.x;
        let y = ae * x + be;
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
    if !(0.0..=1.0).contains(&t) {
        // outside corners
        return None;
    }
    let sp = if dp.x == 0.0 {
        (y - p.y) / dp.y
    } else {
        (x - p.x) / dp.x
    };
    let sd = if dp.x == 0.0 {
        (y - (p.y + dp.y)) / dp.y
    } else {
        (x - (p.x + dp.x)) / dp.x
    };
    if sp * sd > 0.0 {
        // wrong side
        return None;
    }
    if sd >= 0.0 {
        // out of range
        return None;
    }
    let intersection = V2::new(x, y);
    let score = figure_out_score(p, dp, intersection);
    Some((intersection, score))
}

fn figure_out_score(pos: V2, delta_pos: V2, intersection: V2) -> f64 {
    // intersection = pos + delta_pos * score
    // (intersection - pos) / delta_pos = score

    if delta_pos.x != 0.0 {
        (intersection.x - pos.x) / delta_pos.x
    } else if delta_pos.y != 0.0 {
        (intersection.y - pos.y) / delta_pos.y
    } else {
        unreachable!(
            "already verified delta_pos != (0, 0) in {}",
            stringify!(point_vec_2p_line_intersect)
        );
    }
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
    assert_eq!(
        point_vec_2p_line_intersect(
            V2::new(10.0, 20.0),
            V2::new(0.0, 10.0),
            V2::new(0.0, 10.0),
            V2::new(30.0, 10.0)
        ),
        None,
    );
    assert_eq!(
        point_vec_2p_line_intersect(
            V2::new(10.0, 10.0),
            V2::new(15.0, 15.0),
            V2::new(0.0, 20.0),
            V2::new(30.0, 20.0)
        ),
        Some((V2::new(20.0, 20.0), 2.0 / 3.0)),
    );
    assert_eq!(
        point_vec_2p_line_intersect(
            V2::new(10.0, 10.0),
            V2::new(5.0, 5.0),
            V2::new(0.0, 20.0),
            V2::new(30.0, 20.0)
        ),
        None,
    );
    assert_eq!(
        point_vec_2p_line_intersect(
            V2::new(10.0, 10.0),
            V2::new(-5.0, -5.0),
            V2::new(0.0, 20.0),
            V2::new(30.0, 20.0)
        ),
        None,
    );
    assert_eq!(
        point_vec_2p_line_intersect(
            V2::new(10.0, 10.0),
            V2::new(-15.0, -15.0),
            V2::new(0.0, 20.0),
            V2::new(30.0, 20.0)
        ),
        None,
    );
    assert_eq!(
        point_vec_2p_line_intersect(
            V2::new(30.0, 10.0),
            V2::new(-20.0, 20.0),
            V2::new(0.0, 10.0),
            V2::new(20.0, 20.0)
        ),
        Some((V2::new(20.0, 20.0), 1.0)),
    );

    assert_eq!(
        point_vec_2p_line_intersect(
            V2::new(10.0, 10.0),
            V2::new(20.0, 15.0).extend(2.0),
            V2::new(30.0, 30.0),
            V2::new(30.0, 0.0),
        ),
        Some((V2::new(20.0, 20.0), 1.0)),
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
        if value > 0.0 {
            Positive
        } else if value == 0.0 {
            Zero
        } else {
            Negative
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
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

    pub fn facing(&self, direction: Direction) -> bool {
        use Direction::*;
        match (direction, self) {
            (Top, Top | TopLeft | TopRight)
            | (Right, Right | TopRight | BottomRight)
            | (Bottom, Bottom | BottomLeft | BottomRight) => true,
            (Left, Left | TopLeft | TopRight) => todo!(),
            (Top | Right | Bottom | Left, _) => false,
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
            (Zero, Positive) => Bottom,
            (Zero, Negative) => Top,
            (Positive, Zero) => Right,
            (Negative, Zero) => Left,
            (Positive, Positive) => BottomRight,
            (Positive, Negative) => TopRight,
            (Negative, Positive) => BottomLeft,
            (Negative, Negative) => TopLeft,
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

fn resolve_collision(body: &mut RigidBody, p: V2, rect: V2, dir: Direction) {
    use Direction::*;
    match dir {
        Top => {
            body.pos.1 = p.y + 1.0;
            body.vel.1 = 0.0;
        }
        Bottom => {
            body.pos.1 = p.y - rect.y - 1.0;
            body.vel.1 = 0.0;
        }
        Left => {
            body.pos.0 = p.x + 1.0;
            body.vel.0 = 0.0;
        }
        Right => {
            body.pos.0 = p.x - rect.x - 1.0;
            body.vel.0 = 0.0;
        }
        _ => unreachable!(),
    }
}

#[derive(Component, Clone, Default)]
pub struct Collider {
    pub resolve: bool,
    pub colliding: Option<Direction>,
}

pub struct CollisionSystem;
impl System for CollisionSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody, Collider) {
            let collider = ctx.entity_component::<Collider>(id).clone();
            if !collider.resolve {
                continue;
            }
            let collider = ctx.entity_component::<Collider>(id);
            collider.colliding = None;
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

                let mut ints = Vec::<(V2, Direction, f64)>::new();

                match Direction::from(dp) {
                    dir @ Direction::None => {
                        let collider = ctx.entity_component::<Collider>(id);
                        collider.colliding = Some(dir);
                    }
                    dir @ (Direction::Top
                    | Direction::Right
                    | Direction::Bottom
                    | Direction::Left) => {
                        let (p0, p1) = rect_side_corners(pos, rect, dir);
                        let (c0, c1) = rect_side_corners(other_pos, other_rect, dir.reverse());
                        for p in [p0, p1] {
                            if let Some((int, t)) = point_vec_2p_line_intersect(p, dp, c0, c1) {
                                ints.push((int, dir, t));
                            }
                        }
                        for p in [c0, c1] {
                            if let Some((_int, t)) =
                                point_vec_2p_line_intersect(p, dp.reverse(), p0, p1)
                            {
                                ints.push((p, dir, t));
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
                                if let Some((int, t)) = point_vec_2p_line_intersect(p, dp, c0, c1) {
                                    ints.push((int, dir, t));
                                }
                            }
                        }
                        for p in [c0, c1, c2] {
                            for (c0, c1, dir) in [(p0, p1, d0), (p1, p2, d1)] {
                                if let Some((_int, t)) =
                                    point_vec_2p_line_intersect(p, dp.reverse(), c0, c1)
                                {
                                    ints.push((p, dir, t));
                                }
                            }
                        }
                    }
                }
                if let Some((p, dir, _)) = ints
                    .into_iter()
                    .min_by(|(.., t0), (.., t1)| t0.total_cmp(t1))
                {
                    let collider = ctx.entity_component::<Collider>(id);
                    collider.colliding = Some(dir);
                    let body = ctx.entity_component::<RigidBody>(id);
                    resolve_collision(body, p, rect, dir)
                }
            }
        }
        Ok(())
    }
}
