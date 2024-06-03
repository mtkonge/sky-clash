use std::collections::HashSet;

use crate::{
    max, min,
    physics::{Line, OctoDirection, QuadDirection, Rect},
    query,
    rigid_body::RigidBody,
    Component, Context, Error, Id, System, V2,
};

fn rects_within_reach(rect: Rect, delta_pos: V2, other_rect: Rect) -> bool {
    let radii = rect.radius() + delta_pos.len() + other_rect.radius();
    let length_between = rect.radii_distance_to_rect(other_rect);
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

fn point_vec_line_intersect(pos: V2, delta_pos: V2, line: Line) -> Option<V2> {
    let line_direction = line.direction();
    if delta_pos.x == 0.0 && line_direction.x == 0.0 {
        // parallel, do nothing
        None
    } else if delta_pos.x == 0.0 {
        let x = pos.x;
        // y = ax + b
        let line_a = line_direction.y / line_direction.x;
        let line_b = line.p0.y - line_a * line.p0.x;
        let y = line_a * x + line_b;
        Some(V2::new(x, y))
    } else if line_direction.x == 0.0 {
        let x = line.p0.x;
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
        let line_b = line.p0.y - line_a * line.p0.x;
        let x = (line_b - delta_pos_b) / (delta_pos_a - line_a);
        let y = delta_pos_a * x + delta_pos_b;
        Some(V2::new(x, y))
    }
}

fn line_point_within_segment(line: Line, intersection: V2) -> bool {
    // x = x0 + t * xr
    // y = y0 + t * yr
    let t = if line.is_vertical() {
        (intersection.y - line.p0.y) / (line.p1.y - line.p0.y)
    } else {
        (intersection.x - line.p0.x) / (line.p1.x - line.p0.x)
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

fn point_vec_line_segment_intersect(pos: V2, delta_pos: V2, line: Line) -> Option<(V2, f64)> {
    if delta_pos.len() == 0.0 {
        // no movement, no collision
        return None;
    }
    let intersection = point_vec_line_intersect(pos, delta_pos, line)?;
    if !line_point_within_segment(line, intersection) {
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
                    point_vec_line_segment_intersect(pos, delta_pos, Line::new(edge_p0, edge_p1))
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

fn resolve_collision(body: &mut RigidBody, p: V2, rect: V2, dir: OctoDirection) {
    use OctoDirection::*;
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

fn bounce_collision(body: &mut RigidBody, p: V2, rect: V2, dir: OctoDirection) {
    use OctoDirection::*;
    if body.vel.len() <= 1200.0 {
        return resolve_collision(body, p, rect, dir);
    }
    match dir {
        Top => {
            body.pos.y = p.y + 1.0;
            body.vel.y = -(body.vel.y / 2.0);
        }
        Bottom => {
            body.pos.y = p.y - rect.y - 1.0;
            body.vel.y = -(body.vel.y / 2.0);
        }
        Left => {
            body.pos.x = p.x + 1.0;
            body.vel.x = -(body.vel.x / 2.0);
        }
        Right => {
            body.pos.x = p.x - rect.x - 1.0;
            body.vel.x = -(body.vel.x / 2.0);
        }
        _ => unreachable!(),
    }
}

#[derive(Component, Clone)]
pub struct ShallowCollider {
    directions: HashSet<QuadDirection>,
}

impl ShallowCollider {
    pub fn new() -> Self {
        Self {
            directions: HashSet::new(),
        }
    }

    pub fn with_direction(mut self, dir: QuadDirection) -> Self {
        self.directions.insert(dir);
        self
    }
}

#[derive(Component, Clone)]
pub struct SolidCollider {
    pub resolve: bool,
    pub bounce: bool,
    pub colliding: Option<OctoDirection>,
    pub size: Option<V2>,
    pub offset: V2,
}

impl SolidCollider {
    pub fn new() -> Self {
        Self {
            resolve: false,
            bounce: false,
            colliding: None,
            size: None,
            offset: V2::new(0.0, 0.0),
        }
    }

    pub fn bouncing(self) -> Self {
        Self {
            bounce: true,
            ..self
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

struct Intersection {
    pos: V2,
    direction: OctoDirection,
    delta_pos_percentage: f64,
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

            let mut collisions = Vec::<Intersection>::new();
            shallow_intersections(&mut collisions, ctx, id, body.clone(), delta);
            solid_intersections(&mut collisions, ctx, id, body.clone(), collider, delta);

            let size = V2::from(body.clone().size);

            collisions.sort_by(|a, b| a.delta_pos_percentage.total_cmp(&b.delta_pos_percentage));

            let horizontal_collisions = collisions
                .iter()
                .filter(|c| match c.direction {
                    OctoDirection::Left | OctoDirection::Right => true,
                    OctoDirection::Top | OctoDirection::Bottom => false,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();

            let vertical_collisions = collisions
                .iter()
                .filter(|c| match c.direction {
                    OctoDirection::Left | OctoDirection::Right => false,
                    OctoDirection::Top | OctoDirection::Bottom => true,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();

            if let Some(int) = horizontal_collisions.first() {
                let collider = ctx.select::<SolidCollider>(id);
                collider.colliding = Some(int.direction);
                if collider.bounce {
                    let body = ctx.select::<RigidBody>(id);
                    bounce_collision(body, int.pos, size, int.direction)
                } else {
                    let body = ctx.select::<RigidBody>(id);
                    resolve_collision(body, int.pos, size, int.direction);
                }
            }
            if let Some(int) = vertical_collisions.first() {
                let collider = ctx.select::<SolidCollider>(id);
                collider.colliding = Some(int.direction);
                if collider.bounce {
                    let body = ctx.select::<RigidBody>(id);
                    bounce_collision(body, int.pos, size, int.direction)
                } else {
                    let body = ctx.select::<RigidBody>(id);
                    resolve_collision(body, int.pos, size, int.direction);
                }
            }
        }
        Ok(())
    }
}

fn solid_intersections(
    intersections: &mut Vec<Intersection>,
    ctx: &mut Context,
    id: u64,
    body: RigidBody,
    collider: SolidCollider,
    delta: f64,
) {
    'colliders_loop: for other_id in query!(ctx, RigidBody, SolidCollider) {
        if id == other_id {
            continue;
        }

        let other_collider = ctx.select::<SolidCollider>(other_id).clone();
        if other_collider.resolve && collider.resolve {
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

        for direction in [
            QuadDirection::Top,
            QuadDirection::Right,
            QuadDirection::Bottom,
            QuadDirection::Left,
        ] {
            let (p0, p1) = Rect::new(pos, size).side_corners(direction);
            let (c0, c1) = Rect::new(other_pos, other_size).side_corners(direction.reverse());
            for p in [p0, p1] {
                if let Some((int, t)) =
                    point_vec_line_segment_intersect(p, delta_pos, Line::new(c0, c1))
                {
                    intersections.push(Intersection {
                        pos: int,
                        direction: direction.into(),
                        delta_pos_percentage: t,
                    });
                    continue 'colliders_loop;
                }
            }
            for p in [c0, c1] {
                if let Some((_int, t)) =
                    point_vec_line_segment_intersect(p, delta_pos, Line::new(p0, p1))
                {
                    intersections.push(Intersection {
                        pos: p,
                        direction: direction.into(),
                        delta_pos_percentage: t,
                    });
                    continue 'colliders_loop;
                }
            }
        }
    }
}

fn correct_delta_pos(side: OctoDirection, delta_pos: V2) -> bool {
    side == OctoDirection::Top && delta_pos.y.is_sign_positive()
        || side == OctoDirection::Bottom && delta_pos.y.is_sign_negative()
        || side == OctoDirection::Right && delta_pos.x.is_sign_negative()
        || side == OctoDirection::Left && delta_pos.x.is_sign_positive()
}

fn shallow_intersections(
    intersections: &mut Vec<Intersection>,
    ctx: &mut Context,
    id: Id,
    body: RigidBody,
    delta: f64,
) {
    'colliders_loop: for other_id in query!(ctx, RigidBody, ShallowCollider) {
        let other_body = ctx.select::<RigidBody>(other_id).clone();
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

        let other_collider = ctx.select::<ShallowCollider>(other_id).clone();

        for side in [
            QuadDirection::Top,
            QuadDirection::Right,
            QuadDirection::Bottom,
            QuadDirection::Left,
        ] {
            if other_collider.directions.contains(&side.into())
                && correct_delta_pos(side.into(), delta_pos)
            {
                let (p0, p1) = Rect::new(pos, size).side_corners(side.reverse());
                let (c0, c1) = Rect::new(other_pos, other_size).side_corners(side);
                for p in [p0, p1] {
                    if let Some((int, t)) =
                        point_vec_line_segment_intersect(p, delta_pos, Line::new(c0, c1))
                    {
                        intersections.push(Intersection {
                            pos: int,
                            direction: side.reverse().into(),
                            delta_pos_percentage: t,
                        });
                        continue 'colliders_loop;
                    }
                }
                for p in [c0, c1] {
                    if let Some((_int, t)) =
                        point_vec_line_segment_intersect(p, delta_pos.reverse(), Line::new(p0, p1))
                    {
                        intersections.push(Intersection {
                            pos: p,
                            direction: side.reverse().into(),
                            delta_pos_percentage: t,
                        });
                        continue 'colliders_loop;
                    }
                }
            }
        }
    }
}
