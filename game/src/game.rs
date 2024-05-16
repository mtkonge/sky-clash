use engine::{
    rigid_body::{DragSystem, GravitySystem, RigidBody, VelocitySystem},
    spawn, Collider, CollisionSystem, Component, System,
};

use crate::{
    hurtbox::{self, HurtDirection, Hurtbox, HurtboxSystem, Victim},
    key_set::KeySet,
    knockoff::KnockoffSystem,
    player_attack::{PlayerAttack, PlayerAttackSystem},
    player_movement::{PlayerMovement, PlayerMovementSystem},
    sprite_renderer::{Sprite, SpriteRenderer},
};

pub struct GameSystem(pub u64);

#[derive(Component, Clone)]
pub struct HeroesOnBoard {
    pub hero_1: shared::Hero,
    pub hero_2: shared::Hero,
}

impl System for GameSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        ctx.add_system(CollisionSystem);
        ctx.add_system(VelocitySystem);
        ctx.add_system(SpriteRenderer);
        ctx.add_system(PlayerMovementSystem);
        ctx.add_system(GravitySystem);
        ctx.add_system(DragSystem);
        ctx.add_system(HurtboxSystem);
        ctx.add_system(KnockoffSystem);
        ctx.add_system(PlayerAttackSystem);
        let heroes = ctx.clone_one::<HeroesOnBoard>();
        let hero_1_sprite = {
            let path = crate::hero_info::HeroInfo::from(&heroes.hero_1.hero_type).texture_path;
            ctx.load_texture(path).unwrap()
        };
        let hero_2_sprite = {
            let path = crate::hero_info::HeroInfo::from(&heroes.hero_2.hero_type).texture_path;
            ctx.load_texture(path).unwrap()
        };

        let background = ctx.load_texture("textures/literally_dprk.png").unwrap();
        let nope = ctx.load_texture("textures/nuh-uh.png").unwrap();

        spawn!(
            ctx,
            Sprite { sprite: background },
            RigidBody {
                rect: (1280.0, 720.0),
                ..Default::default()
            },
        );

        spawn!(
            ctx,
            Sprite {
                sprite: hero_1_sprite
            },
            RigidBody {
                pos: (400.0, 200.0),
                rect: (128.0, 128.0),
                gravity: true,
                drag: true,
                ..Default::default()
            },
            Collider {
                resolve: true,
                ..Default::default()
            },
            PlayerMovement {
                key_set: KeySet::Wasd
            },
            hurtbox::MatchHero {
                kind: hurtbox::PlayerKind::Left,
                hero: heroes.hero_1.clone(),
                knockback_modifier: 5.5,
                lives: 3,
            },
            PlayerAttack {
                key_set: KeySet::Wasd
            },
            Victim::default()
        );

        spawn!(
            ctx,
            Sprite {
                sprite: hero_2_sprite
            },
            RigidBody {
                pos: (600.0, 200.0),
                rect: (128.0, 128.0),
                gravity: true,
                drag: true,
                ..Default::default()
            },
            Collider {
                resolve: true,
                ..Default::default()
            },
            PlayerMovement {
                key_set: KeySet::ArrowKeys
            },
            hurtbox::MatchHero {
                kind: hurtbox::PlayerKind::Right,
                hero: heroes.hero_2.clone(),
                knockback_modifier: 0.0,
                lives: 3,
            },
            PlayerAttack {
                key_set: KeySet::ArrowKeys
            },
            Victim::default(),
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
                pos: (800.0, 200.0),
                rect: (32.0, 32.0),
                ..Default::default()
            },
            Hurtbox {
                direction: HurtDirection::Left,
                power: 20.0,
                ..Default::default()
            },
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
