use crate::{
    engine::{self, Component, System},
    query, RigidBody,
};

type V2 = (f64, f64);

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Diagonal {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

enum DiagonalCommonResult {
    None,
    Direction(Direction),
    Diagonal(Diagonal),
}

impl Diagonal {
    pub fn common(&self, other: &Diagonal) -> DiagonalCommonResult {
        use Diagonal::*;
        use DiagonalCommonResult as R;
        use Direction::*;
        match (self, other) {
            (TopLeft, TopRight) => R::Direction(Top),
            (TopLeft, BottomLeft) => R::Direction(Left),
            (TopRight, TopLeft) => R::Direction(Top),
            (TopRight, BottomRight) => R::Direction(Right),
            (BottomRight, TopRight) => R::Direction(Right),
            (BottomRight, BottomLeft) => R::Direction(Bottom),
            (BottomLeft, TopLeft) => R::Direction(Left),
            (BottomLeft, BottomRight) => R::Direction(Bottom),
            (left, right) if left == right => R::Diagonal(left.clone()),
            _ => R::None,
        }
    }
    pub fn contains(&self, dir: Direction) -> bool {
        match (self, dir) {
            (Diagonal::TopLeft, Direction::Top)
            | (Diagonal::TopLeft, Direction::Left)
            | (Diagonal::TopRight, Direction::Top)
            | (Diagonal::TopRight, Direction::Left)
            | (Diagonal::TopRight, Direction::Right)
            | (Diagonal::BottomRight, Direction::Bottom)
            | (Diagonal::BottomRight, Direction::Right)
            | (Diagonal::BottomLeft, Direction::Bottom)
            | (Diagonal::BottomLeft, Direction::Left) => true,
            (Diagonal::TopLeft, Direction::Bottom)
            | (Diagonal::TopLeft, Direction::Right)
            | (Diagonal::TopRight, Direction::Bottom)
            | (Diagonal::BottomRight, Direction::Top)
            | (Diagonal::BottomRight, Direction::Left)
            | (Diagonal::BottomLeft, Direction::Top)
            | (Diagonal::BottomLeft, Direction::Right) => false,
        }
    }
}

fn point_rect_closest_point(pos: V2, other_pos: V2, rect: V2) -> V2 {
    [
        other_pos,
        (other_pos.0, other_pos.1 + rect.1),
        (other_pos.0 + rect.0, other_pos.1),
        (other_pos.0 + rect.0, other_pos.1 + rect.1),
    ]
    .into_iter()
    .map(|p| (p, point_distance(pos, p)))
    .min_by(|(_, a), (_, b)| a.total_cmp(b))
    .map(|(p, _)| p)
    .unwrap()
}

fn rect_adjacent_corners(pos: V2, rect: V2, corner: V2) -> (V2, (f64, f64)) {
    if corner == pos {
        ((pos.0, pos.1 + rect.1), (pos.0 + rect.0, pos.1))
    } else if corner == (pos.0 + rect.0, pos.1) {
        (pos, (pos.0 + rect.0, pos.1 + rect.1))
    } else if corner == (pos.0 + rect.0, pos.1 + rect.1) {
        ((pos.0 + rect.0, pos.1), (pos.0, pos.1 + rect.1))
    } else if corner == (pos.0, pos.1 + rect.1) {
        ((pos.0 + rect.0, pos.1), pos)
    } else {
        unreachable!()
    }
}

fn line_intersection(p0: V2, r0: V2, p1: V2, r1: V2) -> Option<V2> {
    if r0.0 == 0.0 && r1.0 == 0.0 {
        // both vertical
        return None;
    }
    // y = ax + b
    // a = y / x
    let a0 = r0.1 / r0.0;
    let a1 = r1.1 / r1.0;
    if a0 == a1 {
        // parallel
        return None;
    }
    // b = y - ax
    let b0 = p0.1 - a0 * p0.0;
    let b1 = p1.1 - a1 * p1.0;
    //                 y = a0 * x + b0
    //                 y = a1 * x + b1
    //       a0 * x + b0 = a1 * x + b1
    //   a0 * x - a1 * x = b1 - b0
    //     x * (a0 - a1) = b1 - b0
    //                 x = (b1 - b0) / (a0 - a1)
    let x = (b1 - b0) / (a0 - a1);
    let y = a0 * x + b0;
    Some((x, y))
}

fn point_between_points(p: V2, p0: V2, p1: V2) -> bool {
    let t = (p.0 - p0.0) / (p1.0 - p0.0);
    0.0 < t && t < 1.0
}

fn point_distance(a: V2, b: (f64, f64)) -> f64 {
    ((a.0 - b.0).abs().powi(2) + (a.1 - b.1).abs().powi(2)).sqrt()
}

fn rects_closet_points(
    pos: V2,
    rect: V2,
    other_pos: V2,
    other_rect: V2,
) -> ((V2, Diagonal), (V2, Diagonal)) {
    use Diagonal::*;
    let points = [
        ((pos.0, pos.1), TopLeft),
        ((pos.0 + rect.0, pos.1), TopRight),
        ((pos.0 + rect.0, pos.1 + rect.1), BottomRight),
        ((pos.0, pos.1 + rect.1), BottomLeft),
    ];

    let other_points = [
        ((other_pos.0, other_pos.1), TopLeft),
        ((other_pos.0 + other_rect.0, other_pos.1), TopRight),
        (
            (other_pos.0 + other_rect.0, other_pos.1 + other_rect.1),
            BottomRight,
        ),
        ((other_pos.0, other_pos.1 + other_rect.1), BottomLeft),
    ];

    let mut lowest = (
        f64::INFINITY,
        (((0.0, 0.0), TopLeft), ((0.0, 0.0), TopLeft)),
    );
    for (point, dir) in points {
        for (other_point, other_dir) in other_points.iter() {
            let distance = point_distance(point, *other_point);
            if distance < lowest.0 {
                lowest = (
                    distance,
                    ((point, dir.clone()), (*other_point, other_dir.clone())),
                );
            }
        }
    }
    lowest.1
}

fn point_vel_rect_collision(pos: V2, vel: V2, other_pos: V2, rect: V2) -> Option<V2> {
    let c1 = point_rect_closest_point(pos, other_pos, rect);
    let (c0, c2) = rect_adjacent_corners(other_pos, rect, c1);
    let intersection_c1_c0 = line_intersection(pos, vel, c1, (c0.0 - c1.0, c0.1 - c1.0))?;
    if point_between_points(intersection_c1_c0, c1, c0) {
        return Some(intersection_c1_c0);
    }
    let intersection_c1_c2 = line_intersection(pos, vel, c1, (c2.0 - c1.0, c2.1 - c1.0))?;
    if point_between_points(intersection_c1_c2, c1, c2) {
        return Some(intersection_c1_c2);
    }
    None
}

fn rect_collision(
    pos: V2,
    vel: V2,
    rect: V2,
    other_pos: V2,
    other_rect: V2,
) -> Option<(V2, Direction)> {
    let ((p0, d0), (_, d1)) = rects_closet_points(pos, rect, other_pos, other_rect);
    let common = match d0.common(&d1) {
        DiagonalCommonResult::Direction(dir) => dir,
        _ => return None,
    };
    let new_pos = point_vel_rect_collision(p0, vel, other_pos, other_rect)?;
    Some((new_pos, common))
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
                let Some((new_pos, dir)) = rect_collision(
                    body.pos,
                    (body.vel.0 * delta, body.vel.1 * delta),
                    body.rect,
                    other_body.pos,
                    other_body.rect,
                ) else {
                    continue;
                };
                let body = ctx.entity_component::<RigidBody>(id);
                body.pos = new_pos;
                match dir {
                    Direction::Top | Direction::Bottom => body.vel.0 = 0.0,
                    Direction::Left | Direction::Right => body.vel.1 = 0.0,
                }
            }
        }
        Ok(())
    }
}
