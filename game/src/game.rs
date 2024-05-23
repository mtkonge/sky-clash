use engine::{
    query,
    rigid_body::{DragSystem, GravitySystem, RigidBody, VelocitySystem},
    spawn, Collider, CollisionSystem, Component, System,
};

use crate::{
    hud::HudSystem,
    hurtbox::{Hitbox, Hurtbox, HurtboxSystem, Victim},
    keyset::Keyset,
    knockoff::KnockoffSystem,
    player::{Player, PlayerKind},
    player_interaction::{PlayerInteraction, PlayerInteractionSystem},
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
        ctx.add_system(GravitySystem);
        ctx.add_system(DragSystem);
        ctx.add_system(HurtboxSystem);
        ctx.add_system(KnockoffSystem);
        ctx.add_system(PlayerInteractionSystem);
        ctx.add_system(HudSystem);
        ctx.add_system(DebugDrawer);

        let background = ctx.load_texture("textures/literally_dprk.png").unwrap();
        let nope = ctx.load_texture("textures/nuh-uh.png").unwrap();

        spawn!(
            ctx,
            Sprite::new(background).layer(2),
            RigidBody::new().with_size((1280.0, 720.0)),
        );

        self.spawn_player(ctx, (400.0, 200.0), Keyset::Wasd, PlayerKind::Left);
        self.spawn_player(ctx, (600.0, 200.0), Keyset::ArrowKeys, PlayerKind::Right);

        spawn!(
            ctx,
            RigidBody::new()
                .with_pos((200.0, 400.0))
                .with_size((32.0, 32.0)),
            Collider::new(),
            Sprite::new(nope),
        );

        spawn!(
            ctx,
            RigidBody::new()
                .with_pos((1100.0, 400.0))
                .with_size((32.0, 32.0)),
            Collider::new(),
            Sprite::new(nope),
        );

        spawn!(
            ctx,
            RigidBody::new()
                .with_pos((184.0, 540.0))
                .with_size((960.0, 128.0)),
            Collider::new(),
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

impl GameSystem {
    fn spawn_player(
        &self,
        ctx: &mut engine::Context,
        pos: (f64, f64),
        keyset: Keyset,
        kind: PlayerKind,
    ) {
        let scale = 1.5;
        let pixel_ratio = 4.0;

        let hero = self.player_hero(ctx, &kind);
        let texture = self.hero_texture(ctx, &hero.kind);

        let factor = scale * pixel_ratio;
        spawn!(
            ctx,
            Sprite::new(texture).layer(1),
            Hitbox {
                size: (24.0 * factor, 28.0 * factor),
                offset: (4.0 * factor, 2.0 * factor)
            },
            RigidBody::new()
                .with_pos(pos)
                .with_size((32.0 * factor, 32.0 * factor))
                .with_gravity()
                .with_drag(),
            Collider::new().resolving(),
            Player {
                kind,
                hero,
                knockback_modifier: 0.0,
                lives: 3,
            },
            PlayerInteraction::new(keyset, 0.0),
            Victim::default()
        );
    }

    fn player_hero(&self, ctx: &mut engine::Context, kind: &PlayerKind) -> shared::Hero {
        let heroes = ctx.clone_one::<HeroesOnBoard>();
        match kind {
            PlayerKind::Left => heroes.hero_1,
            PlayerKind::Right => heroes.hero_2,
        }
    }

    fn hero_texture(&self, ctx: &mut engine::Context, kind: &shared::HeroKind) -> engine::Texture {
        let path = crate::hero_info::HeroInfo::from(kind).texture_path;
        ctx.load_texture(path).unwrap()
    }
}

struct DebugDrawer(pub u64);

impl System for DebugDrawer {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody, Collider) {
            let body = ctx.select::<RigidBody>(id).clone();
            self.draw_outline(ctx, body.pos, body.size, 2.0, (0, 125, 255))?;
        }
        for id in query!(ctx, RigidBody, Hurtbox) {
            let body = ctx.select::<RigidBody>(id).clone();
            self.draw_outline(ctx, body.pos, body.size, 2.0, (255, 0, 0))?;
        }
        for id in query!(ctx, RigidBody, Hitbox) {
            let body = ctx.select::<RigidBody>(id).clone();
            let hitbox = ctx.select::<Hitbox>(id).clone();
            self.draw_outline(
                ctx,
                (body.pos.0 + hitbox.offset.0, body.pos.1 + hitbox.offset.1),
                hitbox.size,
                2.0,
                (0, 255, 125),
            )?;
        }
        Ok(())
    }
}

impl DebugDrawer {
    fn draw_outline(
        &self,
        ctx: &mut engine::Context,
        pos: (f64, f64),
        size: (f64, f64),
        width: f64,
        color: (u8, u8, u8),
    ) -> Result<(), engine::Error> {
        ctx.draw_rect(
            color,
            pos.0 as i32,
            pos.1 as i32,
            size.0 as u32,
            width as u32,
        )?;
        ctx.draw_rect(
            color,
            (pos.0 + size.0 - width) as i32,
            pos.1 as i32,
            width as u32,
            size.1 as u32,
        )?;
        ctx.draw_rect(
            color,
            pos.0 as i32,
            pos.1 as i32,
            width as u32,
            size.1 as u32,
        )?;
        ctx.draw_rect(
            color,
            pos.0 as i32,
            (pos.1 + size.1 - width) as i32,
            size.0 as u32,
            width as u32,
        )?;
        Ok(())
    }
}
