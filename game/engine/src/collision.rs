use std::{collections::HashSet, ops::ControlFlow, rc::Rc};

use crate::{
    max, min,
    physics::{Intersection, Line, Movable, Moving, OctoDirection, QuadDirection, Rect},
    query,
    rigid_body::RigidBody,
    Component, Context, Error, Id, System, V2,
};

pub trait CollisionResolver {
    fn resolve(&self, body: &mut RigidBody, pos: V2, size: V2, dir: QuadDirection);
}

pub struct DefaultResolver;
impl CollisionResolver for DefaultResolver {
    fn resolve(&self, body: &mut RigidBody, pos: V2, size: V2, dir: QuadDirection) {
        use QuadDirection::*;
        match dir {
            Top => {
                body.pos.y = pos.y;
                body.vel.y = max(0.0, body.vel.y);
            }
            Bottom => {
                body.pos.y = pos.y - size.y;
                body.vel.y = min(0.0, body.vel.y);
            }
            Left => {
                body.pos.x = pos.x;
                body.vel.x = max(0.0, body.vel.x);
            }
            Right => {
                body.pos.x = pos.x - size.x;
                body.vel.x = min(0.0, body.vel.x);
            }
        }
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
    pub resolver: Option<Rc<dyn CollisionResolver>>,
    pub colliding: Option<OctoDirection>,
    pub size: Option<V2>,
    pub offset: V2,
}

impl SolidCollider {
    pub fn new() -> Self {
        Self {
            resolver: None,
            colliding: None,
            size: None,
            offset: V2::new(0.0, 0.0),
        }
    }

    pub fn resolving<R: CollisionResolver + 'static>(self, resolver: R) -> Self {
        Self {
            resolver: Some(Rc::new(resolver)),
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

struct Collision {
    pos: V2,
    direction: QuadDirection,
    distance_factor: f64,
}

pub struct CollisionSystem(pub u64);
impl System for CollisionSystem {
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        use QuadDirection::*;

        for id in query!(ctx, RigidBody, SolidCollider) {
            let collider = ctx.select::<SolidCollider>(id).clone();
            let Some(resolver) = collider.resolver else {
                continue;
            };

            let collider = ctx.select::<SolidCollider>(id);
            collider.colliding = None;

            let body = ctx.select::<RigidBody>(id).clone();

            let mut collisions = Vec::<Collision>::new();
            find_shallow_collisions(&mut collisions, ctx, id, &body, delta);
            find_solid_collisions(&mut collisions, ctx, id, &body, delta);

            collisions.sort_by(|a, b| a.distance_factor.total_cmp(&b.distance_factor));

            let horizontal_collisions = collisions
                .iter()
                .filter(|c| match c.direction {
                    Left | Right => true,
                    Top | Bottom => false,
                })
                .collect::<Vec<_>>();

            let vertical_collisions = collisions
                .iter()
                .filter(|c| match c.direction {
                    Left | Right => false,
                    Top | Bottom => true,
                })
                .collect::<Vec<_>>();

            for collision in [horizontal_collisions.first(), vertical_collisions.first()] {
                if let Some(int) = collision {
                    let collider = ctx.select::<SolidCollider>(id);
                    collider.colliding = Some(int.direction.into());
                    let body = ctx.select::<RigidBody>(id);
                    resolver.resolve(body, int.pos, body.size, int.direction)
                }
            }
        }
        Ok(())
    }
}

fn find_solid_collisions(
    collisions: &mut Vec<Collision>,
    ctx: &mut Context,
    id: u64,
    body: &RigidBody,
    delta: f64,
) {
    for other_id in query!(ctx, RigidBody, SolidCollider) {
        if id == other_id {
            continue;
        }

        let collider = ctx.select::<SolidCollider>(id).clone();
        let other_collider = ctx.select::<SolidCollider>(other_id).clone();
        if other_collider.resolver.is_some() && collider.resolver.is_some() {
            continue;
        }

        let other_body = ctx.select::<RigidBody>(other_id).clone();

        find_collisions(collisions, body, &other_body, delta, |_, _| true);
    }
}

fn correct_delta_pos(side: OctoDirection, delta_pos: V2) -> bool {
    use OctoDirection::*;
    side == Top && delta_pos.y.is_sign_positive()
        || side == Bottom && delta_pos.y.is_sign_negative()
        || side == Right && delta_pos.x.is_sign_negative()
        || side == Left && delta_pos.x.is_sign_positive()
}

fn find_shallow_collisions(
    collisions: &mut Vec<Collision>,
    ctx: &mut Context,
    id: Id,
    body: &RigidBody,
    delta: f64,
) {
    for other_id in query!(ctx, RigidBody, ShallowCollider) {
        if id == other_id {
            continue;
        }

        let other_body = ctx.select::<RigidBody>(other_id).clone();
        let other_collider = ctx.select::<ShallowCollider>(other_id).clone();

        find_collisions(collisions, body, &other_body, delta, |side, delta_pos| {
            other_collider.directions.contains(&side) && correct_delta_pos(side.into(), delta_pos)
        });
    }
}

fn find_collisions<F: Fn(QuadDirection, V2) -> bool>(
    intersections: &mut Vec<Collision>,
    body: &RigidBody,
    other_body: &RigidBody,
    delta: f64,
    direction_checked: F,
) {
    use QuadDirection::*;

    let delta_pos = body.vel.extend(delta);
    let rect = Rect::new(body.pos, body.size).moving(delta_pos);

    let other_rect = Rect::new(other_body.pos, other_body.size);

    if !rect.rect_within_reach(other_rect) {
        return;
    }

    for side in [Top, Right, Bottom, Left] {
        if !direction_checked(side.into(), delta_pos) {
            continue;
        }
        let (p0, p1) = rect.side_corners(side.reverse());
        let (c0, c1) = other_rect.side_corners(side);
        for p in [p0, p1] {
            if let Some(Intersection {
                pos,
                distance_factor,
            }) = p
                .moving(delta_pos)
                .line_segment_intersect(Line::new(c0, c1))
            {
                intersections.push(Collision {
                    pos,
                    direction: side.reverse(),
                    distance_factor,
                });
                return;
            }
        }
        for p in [c0, c1] {
            if let Some(Intersection {
                pos: _,
                distance_factor,
            }) = p
                .moving(delta_pos.reverse())
                .line_segment_intersect(Line::new(p0, p1))
            {
                intersections.push(Collision {
                    pos: p,
                    direction: side.reverse(),
                    distance_factor,
                });
                return;
            }
        }
    }
}
