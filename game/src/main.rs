#![allow(dead_code)]

mod collision;
mod engine;

use engine::{Component, System};

use crate::collision::{Collider, CollisionSystem};

#[derive(Component, Default, Clone, Debug)]
struct RigidBody {
    pos: (f64, f64),
    vel: (f64, f64),
    rect: (f64, f64),
    gravity: bool,
}

struct VelocitySystem;
impl System for VelocitySystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody) {
            let body = ctx.entity_component::<RigidBody>(id);
            body.pos.0 += body.vel.0 * delta;
            body.pos.1 += body.vel.1 * delta;
        }
        Ok(())
    }
}

struct GravitySystem;
impl System for GravitySystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody) {
            let body = ctx.entity_component::<RigidBody>(id);
            if !body.gravity {
                continue;
            }
            body.vel.1 = if body.vel.1 < 800.0 {
                body.vel.1 + 1600.0 * delta
            } else {
                body.vel.1
            };
        }
        Ok(())
    }
}

#[derive(Component)]
struct Sprite {
    sprite: engine::Sprite,
}

struct SpriteRenderer;
impl System for SpriteRenderer {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, Sprite, RigidBody) {
            let body = ctx.entity_component::<RigidBody>(id).clone();
            let sprite = ctx.entity_component::<Sprite>(id).sprite;

            ctx.draw_sprite(sprite, body.pos.0 as i32, body.pos.1 as i32)?;
        }
        Ok(())
    }
}

#[derive(Component)]
struct Cloud;

struct CloudSystem;
impl System for CloudSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        let cloud_amount = ctx.entities_with_component::<Cloud>().len();
        if cloud_amount < 1 {
            let cloud = ctx.load_sprite("textures/clouds.png").unwrap();
            spawn!(
                ctx,
                Cloud,
                Sprite { sprite: cloud },
                RigidBody {
                    pos: (-100.0, 150.0),
                    ..Default::default()
                },
            );
        }

        for id in query!(ctx, Cloud, RigidBody) {
            let body = ctx.entity_component::<RigidBody>(id);
            body.vel.0 = if body.vel.0 < 200.0 {
                body.vel.0 + 200.0 * delta
            } else {
                body.vel.0
            };
        }

        for id in query!(ctx, Cloud, RigidBody) {
            let body = ctx.entity_component::<RigidBody>(id);
            if body.pos.0 > 1400.0 {
                ctx.despawn(id);
            }
        }
        Ok(())
    }
}

#[derive(Component)]
struct PlayerMovement;

struct PlayerMovementSystem;
impl System for PlayerMovementSystem {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, PlayerMovement, RigidBody, Collider) {
            let d_down = ctx.key_pressed(engine::Keycode::D);
            let a_down = ctx.key_pressed(engine::Keycode::A);
            let w_down = ctx.key_pressed(engine::Keycode::W);
            let collider = ctx.entity_component::<Collider>(id).clone();
            let body = ctx.entity_component::<RigidBody>(id);
            body.vel.0 = if d_down && !a_down {
                400.0
            } else if !d_down && a_down {
                -400.0
            } else {
                0.0
            };
            if collider.on_ground && w_down {
                body.vel.1 = -1000.0;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut game = engine::Game::new().unwrap();

    let mut context = game.context();
    context.add_system(CollisionSystem);
    context.add_system(VelocitySystem);
    context.add_system(SpriteRenderer);
    // context.add_system(PlayerMovementSystem);
    context.add_system(GravitySystem);
    context.add_system(CloudSystem);
    let player = context.load_sprite("textures/player.png").unwrap();
    let background = context.load_sprite("textures/literally_dprk.png").unwrap();
    let nope = context.load_sprite("textures/nuh-uh.png").unwrap();

    spawn!(
        &mut context,
        Sprite { sprite: background },
        RigidBody::default(),
    );

    spawn!(
        &mut context,
        Sprite { sprite: player },
        RigidBody {
            pos: (400.0, 200.0),
            vel: (10.0, 0.0),
            rect: (128.0, 128.0),
            gravity: true,
            ..Default::default()
        },
        Collider {
            resolve: true,
            ..Default::default()
        },
        PlayerMovement,
    );

    spawn!(
        &mut context,
        RigidBody {
            pos: (184.0, 540.0),
            rect: (960.0, 128.0),
            ..Default::default()
        },
        Collider::default(),
    );

    // spawn!(
    //     &mut context,
    //     RigidBody {
    //         pos: (300.0, 200.0),
    //         rect: (32.0, 32.0),
    //         ..Default::default()
    //     },
    //     Collider::default(),
    //     Sprite { sprite: nope },
    // );
    //
    // spawn!(
    //     &mut context,
    //     RigidBody {
    //         pos: (900.0, 400.0),
    //         rect: (32.0, 32.0),
    //         ..Default::default()
    //     },
    //     Collider::default(),
    //     Sprite { sprite: nope },
    // );

    game.run();
}
