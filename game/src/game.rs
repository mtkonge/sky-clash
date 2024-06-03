use engine::{
    collision::ShallowCollider,
    physics::QuadDirection,
    query,
    rigid_body::{DragSystem, GravitySystem, RigidBody, VelocitySystem},
    spawn, CollisionSystem, Component, SharedPtr, SolidCollider, System, V2,
};

use crate::{
    hud::{player_damage_color, HudSystem},
    hurtbox::{Hitbox, Hurtbox, HurtboxSystem, Victim},
    keyset::Keyset,
    knockoff::KnockoffSystem,
    player::{Player, PlayerKind},
    player_interaction::{PlayerInteraction, PlayerInteractionSystem},
    server::Server,
    sprite_renderer::{Sprite, SpriteRenderer},
    timer::Timer,
};

#[derive(Component, Clone)]
pub struct Game {
    pub board_colors_timer: SharedPtr<Timer>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board_colors_timer: Timer::new(1.0).into(),
        }
    }
}

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

        let background = ctx.load_texture("textures/map_1.png").unwrap();

        spawn!(ctx, Game::new());

        notify_server_about_player_colors(ctx);

        spawn!(
            ctx,
            Sprite::new(background).layer(2),
            RigidBody::new().with_size(V2::new(1280.0, 720.0)),
        );

        self.spawn_player(ctx, V2::new(400.0, 350.0), Keyset::Wasd, PlayerKind::Left);
        self.spawn_player(
            ctx,
            V2::new(600.0, 350.0),
            Keyset::ArrowKeys,
            PlayerKind::Right,
        );

        spawn!(
            ctx,
            RigidBody::new()
                .with_pos(V2::new(350.0, 525.0))
                .with_size(V2::new(676.0, 110.0)),
            SolidCollider::new(),
        );
        spawn!(
            ctx,
            RigidBody::new()
                .with_pos(V2::new(126.0, 162.0))
                .with_size(V2::new(180.0, 204.0)),
            SolidCollider::new(),
        );
        spawn!(
            ctx,
            RigidBody::new()
                .with_pos(V2::new(720.0, 214.0))
                .with_size(V2::new(248.0, 10.0)),
            ShallowCollider::new().with_direction(QuadDirection::Top),
        );
        spawn!(
            ctx,
            RigidBody::new()
                .with_pos(V2::new(720.0, 214.0))
                .with_size(V2::new(248.0, 10.0)),
            ShallowCollider::new().with_direction(QuadDirection::Top),
        );
        spawn!(
            ctx,
            RigidBody::new()
                .with_pos(V2::new(924.0, 378.0))
                .with_size(V2::new(280.0, 10.0)),
            ShallowCollider::new().with_direction(QuadDirection::Top),
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        let game = ctx.clone_one::<Game>();

        game.board_colors_timer.lock().update(delta);
        if game.board_colors_timer.lock().done() {
            notify_server_about_player_colors(ctx);
            game.board_colors_timer.lock().reset()
        }
        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}

fn notify_server_about_player_colors(ctx: &mut engine::Context) {
    let mut hero_1_color = (255, 255, 255);
    let mut hero_2_color = (255, 255, 255);
    for player_id in query!(ctx, Player).clone() {
        let player = ctx.select::<Player>(player_id).clone();
        match player.kind {
            PlayerKind::Left => hero_1_color = player_damage_color(player.damage_taken),
            PlayerKind::Right => hero_2_color = player_damage_color(player.damage_taken),
        }
    }
    let board_colors = shared::UpdateBoardColorsParams {
        hero_1_color,
        hero_2_color,
    };
    let server = ctx.select_one::<Server>();
    server.update_board_colors(board_colors);
}

impl GameSystem {
    fn spawn_player(&self, ctx: &mut engine::Context, pos: V2, keyset: Keyset, kind: PlayerKind) {
        let scale = 1.0;
        let pixel_ratio = 4.0;

        let hero = self.player_hero(ctx, &kind);
        let texture = self.hero_texture(ctx, &hero.kind);

        let factor = scale * pixel_ratio;
        spawn!(
            ctx,
            Sprite::new(texture).layer(1),
            Hitbox {
                size: V2::new(24.0 * factor, 28.0 * factor),
                offset: V2::new(4.0 * factor, 2.0 * factor)
            },
            RigidBody::new()
                .with_pos(pos)
                .with_size(V2::new(32.0 * factor, 32.0 * factor))
                .with_gravity()
                .with_drag(),
            SolidCollider::new().resolving().bouncing(),
            Player {
                kind,
                hero,
                damage_taken: 0.0,
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
        for id in query!(ctx, RigidBody, SolidCollider) {
            let body = ctx.select::<RigidBody>(id).clone();
            self.draw_outline(ctx, body.pos, body.size, 2.0, (0, 125, 255))?;
        }
        for id in query!(ctx, RigidBody, ShallowCollider) {
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
                body.pos + hitbox.offset,
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
        pos: V2,
        size: V2,
        width: f64,
        color: (u8, u8, u8),
    ) -> Result<(), engine::Error> {
        ctx.draw_rect(
            color,
            pos.x as i32,
            pos.y as i32,
            size.x as u32,
            width as u32,
        )?;
        ctx.draw_rect(
            color,
            (pos.x + size.x - width) as i32,
            pos.y as i32,
            width as u32,
            size.y as u32,
        )?;
        ctx.draw_rect(
            color,
            pos.x as i32,
            pos.y as i32,
            width as u32,
            size.y as u32,
        )?;
        ctx.draw_rect(
            color,
            pos.x as i32,
            (pos.y + size.y - width) as i32,
            size.x as u32,
            width as u32,
        )?;
        Ok(())
    }
}
