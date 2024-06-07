use engine::{
    clamp, query, rigid_body::RigidBody, spawn, Component, Context, Error, System, Texture, V2,
};
use shared::Hero;

use crate::{
    hud::{ReturnToMenu, TrashTalk},
    player::Player,
    player_interaction::PlayerInteraction,
    server::Server,
    sound_player::SoundPlayer,
    sprite_renderer::Sprite,
    timer::Timer,
};

pub struct KnockoffSystem(pub u64);
impl System for KnockoffSystem {
    fn on_update(&self, ctx: &mut Context, _delta: f64) -> Result<(), Error> {
        let max_offset_from_screen = 200.0;
        for id in query!(ctx, PlayerInteraction, RigidBody, Player).clone() {
            let rigid_body = ctx.select::<RigidBody>(id).clone();
            if body_outside_area(&rigid_body, max_offset_from_screen) {
                let loser_id = id;
                let player = ctx.select::<Player>(loser_id);
                if player.is_alive() {
                    player.damage_taken = 0.0;
                    player.lives -= 1;
                    let player_pos = rigid_body.pos;
                    let player_size = rigid_body.size;
                    spawn_death_animation(ctx, player_pos, player_size);
                    let sound_player = ctx.select_one::<SoundPlayer>();
                    sound_player.play_effect("assets/sounds/explosion.ogg");
                };
                let player = ctx.select::<Player>(loser_id);
                let player_is_dead = player.is_dead();
                if player_is_dead {
                    let loser_hero = player.hero.clone();
                    let loser_hero_kind = loser_hero.kind.clone();
                    ctx.despawn(loser_id);
                    let winner = ctx.select_one::<Player>().clone();
                    let winner_hero_kind = winner.hero.kind.clone();
                    spawn!(ctx, TrashTalk::new(winner_hero_kind, loser_hero_kind));
                    spawn!(ctx, ReturnToMenu::new());
                    send_match_result(ctx, &winner.hero, &loser_hero);
                    continue;
                }
                let rigid_body = ctx.select::<RigidBody>(loser_id);
                rigid_body.pos = V2::new((1280.0 - rigid_body.size.x) / 2.0, 100.0);
                rigid_body.vel = V2::new(0.0, 0.0);
            }
        }
        Ok(())
    }
}

fn body_outside_area(rigid_body: &RigidBody, max_offset_from_screen: f64) -> bool {
    rigid_body.pos.x + rigid_body.size.x < -max_offset_from_screen
        || rigid_body.pos.x > 1280.0 + max_offset_from_screen
        || rigid_body.pos.y + rigid_body.size.y < -max_offset_from_screen
        || rigid_body.pos.y > 720.0 + max_offset_from_screen
}

fn send_match_result(ctx: &mut Context, winner: &Hero, loser: &Hero) {
    let server = ctx.select_one::<Server>();
    server.create_match(shared::CreateMatchParams {
        winner_hero_id: winner.id,
        loser_hero_id: loser.id,
    });
}

#[derive(Component)]
pub struct DeathAnimation {
    timer: Timer,
    textures: Vec<Texture>,
}

impl DeathAnimation {
    pub fn new(textures: Vec<Texture>) -> Self {
        Self {
            timer: Timer::new(0.5),
            textures,
        }
    }
}

pub struct DeathAnimationSystem(pub u64);
impl System for DeathAnimationSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, Sprite, DeathAnimation) {
            let animation = ctx.select::<DeathAnimation>(id);
            animation.timer.update(delta);

            if animation.timer.done() {
                ctx.despawn(id);
                continue;
            }

            let texture = animation.textures[std::cmp::min(
                ((animation.timer.time_passed() / animation.timer.duration())
                    * animation.textures.len() as f64)
                    .floor() as usize,
                animation.textures.len(),
            )];
            let sprite = ctx.select::<Sprite>(id);
            sprite.texture = texture;
        }
        Ok(())
    }
}

fn spawn_death_animation(ctx: &mut engine::Context, player_pos: V2, player_size: V2) {
    use engine::physics::QuadDirection::*;

    let size = V2::new(30.0, 60.0).extend(8.0);

    let textures = [
        "assets/death_0.png".to_string(),
        "assets/death_1.png".to_string(),
        "assets/death_2.png".to_string(),
        "assets/death_3.png".to_string(),
        "assets/death_4.png".to_string(),
        "assets/death_5.png".to_string(),
        "assets/death_6.png".to_string(),
    ]
    .into_iter()
    .map(|path| ctx.load_texture(path).unwrap())
    .collect::<Vec<_>>();

    let a = 720.0 / 1280.0;
    let above_descending = player_pos.y > player_pos.x * a;
    let above_ascending = player_pos.y > player_pos.x * -a + 720.0;

    let dir = match (above_descending, above_ascending) {
        (true, true) => Bottom,
        (true, false) => Left,
        (false, true) => Right,
        (false, false) => Top,
    };

    let comp_x = size.x / 2.0 - player_size.x / 2.0;
    let comp_y = size.x / 2.0 - player_size.y / 2.0;

    let pos = match dir {
        Top => V2::new(
            clamp(player_pos.x, 0.0, 1280.0 - player_size.x) + size.x - comp_x,
            size.y,
        ),
        Bottom => V2::new(
            clamp(player_pos.x, 0.0, 1280.0 - player_size.x) - comp_x,
            720.0 - size.y,
        ),
        Right => V2::new(
            1280.0 - size.y,
            clamp(player_pos.y, 0.0, 720.0 - size.x) + size.x - comp_y,
        ),
        Left => V2::new(size.y, clamp(player_pos.y, 0.0, 720.0 - size.x) - comp_y),
    };

    let angle = match dir {
        Top => 180.0,
        Right => 270.0,
        Bottom => 0.0,
        Left => 90.0,
    };

    spawn!(
        ctx,
        RigidBody::new().with_pos(pos).with_size(size),
        Sprite::new(textures[0]).angle(angle),
        DeathAnimation::new(textures),
    );
}
