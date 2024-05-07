use engine::{
    rigid_body::{GravitySystem, RigidBody, VelocitySystem},
    spawn, Collider, CollisionSystem, System,
};

use crate::{
    player_movement::{PlayerMovement, PlayerMovementSystem},
    sprite_renderer::{Sprite, SpriteRenderer},
};

pub struct GameSystem(pub u64);

impl System for GameSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        ctx.add_system(CollisionSystem);
        ctx.add_system(VelocitySystem);
        ctx.add_system(SpriteRenderer);
        ctx.add_system(PlayerMovementSystem);
        ctx.add_system(GravitySystem);
        let player = ctx.load_texture("textures/player_outline.png").unwrap();
        let background = ctx.load_texture("textures/black_background.png").unwrap();
        let nope = ctx.load_texture("textures/nuh-uh.png").unwrap();

        spawn!(ctx, Sprite { sprite: background }, RigidBody::default(),);

        spawn!(
            ctx,
            Sprite { sprite: player },
            RigidBody {
                pos: (400.0, 200.0),
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
            ctx,
            RigidBody {
                pos: (184.0, 540.0),
                rect: (960.0, 128.0),
                ..Default::default()
            },
            Collider::default(),
        );

        spawn!(
            ctx,
            RigidBody {
                pos: (250.0, 200.0),
                rect: (32.0, 32.0),
                ..Default::default()
            },
            Collider::default(),
            Sprite { sprite: nope },
        );

        spawn!(
            ctx,
            RigidBody {
                pos: (900.0, 400.0),
                rect: (32.0, 32.0),
                ..Default::default()
            },
            Collider::default(),
            Sprite { sprite: nope },
        );
        Ok(())
    }

    fn on_update(&self, _ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}
