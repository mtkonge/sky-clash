use crate::{max, min, query, Id, V2};
use crate::{rigid_body::RigidBody, Component, Context, Error, System};

struct Rect {
    pub pos: V2,
    pub size: V2,
}

impl Rect {
    #![allow(dead_code)]

    pub fn new(pos: V2, size: V2) -> Self {
        Self { pos, size }
    }

    pub fn from_f64(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            pos: V2::new(x, y),
            size: V2::new(w, h),
        }
    }

    pub fn top_left(&self) -> V2 {
        self.pos
    }

    pub fn top_right(&self) -> V2 {
        self.pos.add_x(self.size.x)
    }

    pub fn bottom_right(&self) -> V2 {
        self.pos + self.size
    }

    pub fn bottom_left(&self) -> V2 {
        self.pos.add_y(self.size.y)
    }

    pub fn radius(&self) -> f64 {
        self.size.div_comps(2.0).len()
    }

    pub fn distance_to_rect(&self, other: Rect) -> f64 {
        (other.pos - self.pos).len() - (self.radius() + other.radius())
    }
}

fn rects_within_reach(rect: Rect, delta_pos: V2, other_rect: Rect) -> bool {
    let radii = rect.radius() + delta_pos.len() + other_rect.radius();
    let length_between = rect.distance_to_rect(other_rect);
    radii >= length_between
}

#[test]
fn test_rects_within_reach() {
    assert!(rects_within_reach(
        Rect::from_f64(0.0, 0.0, 10.0, 0.0),
        V2::new(10.0, 10.0),
        Rect::from_f64(15.0, 0.0, 10.0, 10.0)
    ),);
    assert!(!rects_within_reach(
        Rect::from_f64(0.0, 0.0, 10.0, 0.0),
        V2::new(10.0, 10.0),
        Rect::from_f64(40.0, 0.0, 10.0, 10.0)
    ),);
}

fn point_vec_line_intersect(
    pos: V2,
    delta_pos: V2,
    line_point0: V2,
    line_point1: V2,
) -> Option<V2> {
    let line_direction = line_point1 - line_point0;
    if delta_pos.x == 0.0 && line_direction.x == 0.0 {
        // parallel, do nothing
        None
    } else if delta_pos.x == 0.0 {
        let x = pos.x;
        // y = ax + b
        let line_a = line_direction.y / line_direction.x;
        let line_b = line_point0.y - line_a * line_point0.x;
        let y = line_a * x + line_b;
        Some(V2::new(x, y))
    } else if line_direction.x == 0.0 {
        let x = line_point0.x;
        // y = ax + b
        let delta_pos_a = delta_pos.y / delta_pos.x;
        let delta_pos_b = pos.y - delta_pos_a * pos.x;
        let y = delta_pos_a * x + delta_pos_b;
        Some(V2::new(x, y))
    } else {
        // y = ax + b
        let delta_pos_a = delta_pos.y / delta_pos.x;
        let line_a = line_direction.y / line_direction.x;
        if delta_pos_a == line_a {
            // parallel: either none or continous intersection
            return None;
        }
        let delta_pos_b = pos.y - delta_pos_a * pos.x;
        let line_b = line_point0.y - line_a * line_point0.x;
        let x = (line_b - delta_pos_b) / (delta_pos_a - line_a);
        let y = delta_pos_a * x + delta_pos_b;
        Some(V2::new(x, y))
    }
}

fn line_point_within_segment(line_point0: V2, line_point1: V2, intersection: V2) -> bool {
    // x = x0 + t * xr
    // y = y0 + t * yr
    let t = if line_point1.x == line_point0.x {
        (intersection.y - line_point0.y) / (line_point1.y - line_point0.y)
    } else {
        (intersection.x - line_point0.x) / (line_point1.x - line_point0.x)
    };
    (0.0..=1.0).contains(&t)
}

fn point_vec_crosses_intersection(pos: V2, delta_pos: V2, intersection: V2) -> bool {
    let pos_s = if delta_pos.x == 0.0 {
        (intersection.y - pos.y) / delta_pos.y
    } else {
        (intersection.x - pos.x) / delta_pos.x
    };
    let delta_pos_s = if delta_pos.x == 0.0 {
        (intersection.y - (pos.y + delta_pos.y)) / delta_pos.y
    } else {
        (intersection.x - (pos.x + delta_pos.x)) / delta_pos.x
    };
    if pos_s * delta_pos_s > 0.0 {
        // wrong side
        return false;
    }
    if delta_pos_s >= 0.0 {
        // out of range
        return false;
    }
    true
}

fn distance_factor_to_intersection(pos: V2, delta_pos: V2, intersection: V2) -> f64 {
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

fn point_vec_line_segment_intersect(
    pos: V2,
    delta_pos: V2,
    line_point0: V2,
    line_point1: V2,
) -> Option<(V2, f64)> {
    if delta_pos.len() == 0.0 {
        // no movement, no collision
        return None;
    }
    let intersection = point_vec_line_intersect(pos, delta_pos, line_point0, line_point1)?;
    if !line_point_within_segment(line_point0, line_point1, intersection) {
        return None;
    }
    if !point_vec_crosses_intersection(pos, delta_pos, intersection) {
        return None;
    }
    let score = distance_factor_to_intersection(pos, delta_pos, intersection);
    Some((intersection, score))
}

#[test]
fn test_point_vec_line_segment_intersect() {
    macro_rules! named {
        ($name: ident) => {
            (stringify!($name), $name)
        };
    }
    let check_a = {
        let edge_a = (V2::new(10.0, 10.0), V2::new(40.0, 10.0));
        let line_a = (V2::new(20.0, 0.0), V2::new(10.0, 20.0));
        let line_b = (V2::new(25.0, 0.0), V2::new(0.0, 25.0));
        let line_c = (V2::new(30.0, 0.0), V2::new(-10.0, 20.0));
        let intersection = V2::new(25.0, 10.0);

        [named!(line_a), named!(line_b), named!(line_c)]
            .into_iter()
            .map(|line| (line, named!(edge_a), intersection))
            .collect::<Vec<_>>()
    };
    let check_b = {
        let edge_b = (V2::new(40.0, 40.0), V2::new(40.0, 10.0));
        let line_d = (V2::new(50.0, 20.0), V2::new(-20.0, 10.0));
        let line_e = (V2::new(50.0, 25.0), V2::new(-25.0, 0.0));
        let line_f = (V2::new(50.0, 30.0), V2::new(-20.0, -10.0));
        let intersection = V2::new(40.0, 25.0);

        [named!(line_d), named!(line_e), named!(line_f)]
            .into_iter()
            .map(|line| (line, named!(edge_b), intersection))
            .collect::<Vec<_>>()
    };
    let check_c = {
        let edge_c = (V2::new(40.0, 40.0), V2::new(10.0, 40.0));
        let line_i = (V2::new(20.0, 50.0), V2::new(10.0, -20.0));
        let line_h = (V2::new(25.0, 50.0), V2::new(0.0, -25.0));
        let line_g = (V2::new(30.0, 50.0), V2::new(-10.0, -20.0));
        let intersection = V2::new(25.0, 40.0);

        [named!(line_i), named!(line_h), named!(line_g)]
            .into_iter()
            .map(|line| (line, named!(edge_c), intersection))
            .collect::<Vec<_>>()
    };
    let check_d = {
        let edge_d = (V2::new(10.0, 10.0), V2::new(10.0, 40.0));
        let line_d = (V2::new(0.0, 20.0), V2::new(20.0, 10.0));
        let line_e = (V2::new(0.0, 25.0), V2::new(25.0, 0.0));
        let line_f = (V2::new(0.0, 30.0), V2::new(20.0, -10.0));
        let intersection = V2::new(10.0, 25.0);

        [named!(line_d), named!(line_e), named!(line_f)]
            .into_iter()
            .map(|line| (line, named!(edge_d), intersection))
            .collect::<Vec<_>>()
    };
    [check_a, check_b, check_c, check_d]
        .into_iter()
        .flatten()
        .for_each(
            |(
                (line_name, (pos, delta_pos)),
                (edge_name, (edge_p0, edge_p1)),
                expected_intersection,
            )| {
                let intersection =
                    point_vec_line_segment_intersect(pos, delta_pos, edge_p0, edge_p1)
                        .map(|(intersection, _score)| intersection);

                assert!(
                    intersection.is_some(),
                    "expected line {line_name} to intersect with edge {edge_name}, got None"
                );

                let intersection = intersection.expect("we asserted it to be Some");
                assert_eq!(intersection, expected_intersection, "expected line {line_name} to intersect with edge {edge_name} at {expected_intersection:?}, got {intersection:?}")
            },
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

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum Direction {
    None = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Left = 4,
    TopLeft = 5,
    TopRight = 6,
    BottomRight = 7,
    BottomLeft = 8,
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

#[derive(Component, Clone)]
struct Directions {
    flags: i32,
}

impl Directions {
    pub fn new() -> Self {
        Self { flags: 0 }
    }

    pub fn set(&mut self, dir: Direction) {
        self.flags |= 1 << dir as i32;
    }

    pub fn unset(&mut self, dir: Direction) {
        self.flags &= !(1 << dir as i32);
    }

    pub fn is_set(&self, dir: Direction) -> bool {
        (self.flags & 1 << dir as i32) != 0
    }
}

#[test]
fn test_directions() {
    let mut dirs = Directions::new();

    dirs.set(Direction::Top);
    dirs.set(Direction::Left);

    assert_eq!(dirs.is_set(Direction::Top), true);
    assert_eq!(dirs.is_set(Direction::Left), true);
    assert_eq!(dirs.is_set(Direction::Right), false);
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
            body.pos.y = p.y + 1.0;
            body.vel.y = max(0.0, body.vel.y);
        }
        Bottom => {
            body.pos.y = p.y - rect.y - 1.0;
            body.vel.y = min(0.0, body.vel.y);
        }
        Left => {
            body.pos.x = p.x + 1.0;
            body.vel.x = max(0.0, body.vel.x);
        }
        Right => {
            body.pos.x = p.x - rect.x - 1.0;
            body.vel.x = min(0.0, body.vel.x);
        }
        _ => unreachable!(),
    }
}

#[derive(Component, Clone)]
pub struct ShallowCollider {
    directions: Directions,
}

impl ShallowCollider {
    pub fn new() -> Self {
        Self {
            directions: Directions::new(),
        }
    }

    pub fn with_direction(mut self, dir: Direction) -> Self {
        self.directions.set(dir);
        self
    }
}

#[derive(Component, Clone)]
pub struct SolidCollider {
    pub resolve: bool,
    pub colliding: Option<Direction>,
    pub size: Option<V2>,
    pub offset: V2,
}

impl SolidCollider {
    pub fn new() -> Self {
        Self {
            resolve: false,
            colliding: None,
            size: None,
            offset: V2::new(0.0, 0.0),
        }
    }

    pub fn resolving(self) -> Self {
        Self {
            resolve: true,
            ..self
        }
    }

    pub fn size(self, size: V2) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }
    pub fn offset(self, offset: V2) -> Self {
        Self { offset, ..self }
    }
}

pub struct CollisionSystem(pub u64);
impl System for CollisionSystem {
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        for id in query!(ctx, RigidBody, SolidCollider) {
            let collider = ctx.select::<SolidCollider>(id).clone();
            if !collider.resolve {
                continue;
            }
            let collider = ctx.select::<SolidCollider>(id);
            collider.colliding = None;
            let body = ctx.select::<RigidBody>(id).clone();
            let collider = ctx.select::<SolidCollider>(id).clone();

            let mut collisions = Vec::<(V2, Direction, f64)>::new();
            collisions.append(&mut shallow_intersections(ctx, id, &body, collider, delta));
            // collisions.push(solid_intersections());
            // resolve_collisions(collisions);

            // handle_shallow_collisions(ctx, delta, id, &body);

            for other_id in query!(ctx, RigidBody, SolidCollider) {
                if id == other_id {
                    continue;
                }
                let other_collider = ctx.select::<SolidCollider>(other_id).clone();
                if other_collider.resolve {
                    continue;
                }
                let other_body = ctx.select::<RigidBody>(other_id).clone();

                let pos = V2::from(body.pos);
                let size = V2::from(body.size);
                let other_pos = V2::from(other_body.pos);
                let other_size = V2::from(other_body.size);
                let delta_pos = V2::from(body.vel).extend(delta);

                if !rects_within_reach(
                    Rect::new(pos, size),
                    delta_pos,
                    Rect::new(other_pos, other_size),
                ) {
                    continue;
                }

                let mut intersections = Vec::<(V2, Direction, f64)>::new();

                match Direction::from(delta_pos) {
                    dir @ Direction::None => {
                        let collider = ctx.select::<SolidCollider>(id);
                        collider.colliding = Some(dir);
                    }
                    dir @ (Direction::Top
                    | Direction::Right
                    | Direction::Bottom
                    | Direction::Left) => {
                        let (p0, p1) = rect_side_corners(pos, size, dir);
                        let (c0, c1) = rect_side_corners(other_pos, other_size, dir.reverse());
                        for p in [p0, p1] {
                            if let Some((int, t)) =
                                point_vec_line_segment_intersect(p, delta_pos, c0, c1)
                            {
                                intersections.push((int, dir, t));
                            }
                        }
                        for p in [c0, c1] {
                            if let Some((_int, t)) =
                                point_vec_line_segment_intersect(p, delta_pos.reverse(), p0, p1)
                            {
                                intersections.push((p, dir, t));
                            }
                        }
                    }
                    dir @ (Direction::TopLeft
                    | Direction::TopRight
                    | Direction::BottomRight
                    | Direction::BottomLeft) => {
                        let (p0, p1, p2) = rect_diagonal_corners(pos, size, dir);
                        let (c0, c1, c2) =
                            rect_diagonal_corners(other_pos, other_size, dir.reverse());
                        let (d0, d1) = dir.clockwise();
                        for p in [p0, p1, p2] {
                            for (c0, c1, dir) in [(c0, c1, d0), (c1, c2, d1)] {
                                if let Some((int, t)) =
                                    point_vec_line_segment_intersect(p, delta_pos, c0, c1)
                                {
                                    intersections.push((int, dir, t));
                                }
                            }
                        }
                        for p in [c0, c1, c2] {
                            for (c0, c1, dir) in [(p0, p1, d0), (p1, p2, d1)] {
                                if let Some((_int, t)) =
                                    point_vec_line_segment_intersect(p, delta_pos.reverse(), c0, c1)
                                {
                                    intersections.push((p, dir, t));
                                }
                            }
                        }
                    }
                }
                if let Some((p, dir, _)) = intersections
                    .into_iter()
                    .min_by(|(.., t0), (.., t1)| t0.total_cmp(t1))
                {
                    let collider = ctx.select::<SolidCollider>(id);
                    collider.colliding = Some(dir);
                    let body = ctx.select::<RigidBody>(id);
                    resolve_collision(body, p, size, dir)
                }
            }
            // split up into to statements one for x axis and on for y axis. the one with the lowest t value should resolve first

            let size = V2::from(body.size);
            collisions.into_iter().for_each(|int| {
                let collider = ctx.select::<SolidCollider>(id);
                collider.colliding = Some(int.1);
                let body = ctx.select::<RigidBody>(id);
                resolve_collision(body, int.0, size, int.1)
            });
        }
        Ok(())
    }
}

fn shallow_intersections(
    ctx: &mut Context,
    id: Id,
    body: &RigidBody,
    collider: SolidCollider,
    delta: f64,
) -> Vec<(V2, Direction, f64)> {
    let mut intersections = Vec::<(V2, Direction, f64)>::new();
    for other_id in query!(ctx, RigidBody, ShallowCollider) {
        let other_body = ctx.select::<RigidBody>(other_id).clone();
        let other_collider = ctx.select::<ShallowCollider>(other_id).clone();
        if id == other_id {
            continue;
        }

        let pos = V2::from(body.pos);
        let size = V2::from(body.size);
        let other_pos = V2::from(other_body.pos);
        let other_size = V2::from(other_body.size);
        let delta_pos = V2::from(body.vel).extend(delta);

        if !rects_within_reach(
            Rect::new(pos, size),
            delta_pos,
            Rect::new(other_pos, other_size),
        ) {
            continue;
        }

        for side in [
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ] {
            if other_collider.directions.is_set(side) {
                let (p0, p1) = rect_side_corners(pos, size, side.reverse());
                let (c0, c1) = rect_side_corners(other_pos, other_size, side);
                for p in [p0, p1] {
                    if let Some((int, t)) = point_vec_line_segment_intersect(p, delta_pos, c0, c1) {
                        intersections.push((int, side.reverse(), t));
                    }
                }
                for p in [c0, c1] {
                    if let Some((_int, t)) =
                        point_vec_line_segment_intersect(p, delta_pos.reverse(), p0, p1)
                    {
                        intersections.push((p, side.reverse(), t));
                    }
                }
            }
        }
    }
    intersections
}
