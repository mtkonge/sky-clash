use std::collections::HashSet;

use crate::{
    max, min,
    physics::{Line, Movable, Moving, OctoDirection, QuadDirection, Rect},
    query,
    rigid_body::RigidBody,
    Component, Context, Error, Id, System, V2,
};

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

        let delta_pos = body.vel.extend(delta);
        let rect = Rect::new(body.pos, body.size).moving(delta_pos);

        let other_body = ctx.select::<RigidBody>(other_id).clone();
        let other_rect = Rect::new(other_body.pos, other_body.size);

        if !rect.rect_within_reach(other_rect) {
            continue;
        }

        for direction in [
            QuadDirection::Top,
            QuadDirection::Right,
            QuadDirection::Bottom,
            QuadDirection::Left,
        ] {
            let (p0, p1) = rect.side_corners(direction);
            let (c0, c1) = other_rect.side_corners(direction.reverse());
            for p in [p0, p1] {
                if let Some((int, t)) = p
                    .moving(delta_pos)
                    .line_segment_intersect(Line::new(c0, c1))
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
                if let Some((_int, t)) = p
                    .moving(delta_pos)
                    .line_segment_intersect(Line::new(p0, p1))
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

        let delta_pos = body.vel.extend(delta);
        let rect = Rect::new(body.pos, body.size).moving(delta_pos);
        let other_rect = Rect::new(other_body.pos, other_body.size);

        if !rect.rect_within_reach(other_rect) {
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
                let (p0, p1) = rect.side_corners(side.reverse());
                let (c0, c1) = other_rect.side_corners(side);
                for p in [p0, p1] {
                    if let Some((int, t)) = p
                        .moving(delta_pos)
                        .line_segment_intersect(Line::new(c0, c1))
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
                    if let Some((_int, t)) = p
                        .moving(delta_pos.reverse())
                        .line_segment_intersect(Line::new(p0, p1))
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
