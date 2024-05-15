use engine::{query, rigid_body::RigidBody, spawn, Component, Context, Error, System};

use crate::{
    hurtbox::{MatchHero, PlayerKind},
    player_movement::PlayerMovement,
};

fn player_died_text(loser: &shared::HeroKind, winner: &shared::HeroKind, counter: f64) -> String {
    let counter = counter as u8 % 3;
    match counter {
        0 => format!("looks like {loser} has skill issues"),
        1 => format!("{loser} was not Him"),
        2 => format!("bro lost to a {winner}"),
        _ => unreachable!(),
    }
}

#[derive(Component, Clone)]
pub struct TrashTalkOffset(f64);

fn win_condition(ctx: &mut Context, loser_id: engine::Id) {
    let winner = 'winner: {
        for winner_id in query!(ctx, PlayerMovement, RigidBody, MatchHero) {
            if winner_id == loser_id {
                continue;
            }
            break 'winner ctx.select::<MatchHero>(winner_id).hero.hero_type.clone();
        }
        unreachable!("other player somehow despawned");
    };
    let trash_talk_offset = ctx.select_one::<TrashTalkOffset>().0;
    let loser = &ctx.select::<MatchHero>(loser_id).hero.hero_type;
    let trash_talk = player_died_text(loser, &winner, trash_talk_offset);
    let font = ctx.load_font("textures/ttf/OpenSans.ttf", 48).unwrap();
    let text = ctx.render_text(font, &trash_talk, (255, 255, 255)).unwrap();
    ctx.draw_texture(text.texture, (1280 - text.size.0) / 2, 100)
        .unwrap();
}

fn draw_match_stats_background(
    ctx: &mut Context,
    border_color: (u8, u8, u8),
    border_thickness: i32,
    pos: (i32, i32),
    text_offset: (i32, i32),
    text_size: (i32, i32),
    stats_size: (u32, u32),
) {
    let text_padding = 5;

    ctx.draw_rect(
        border_color,
        pos.0 - border_thickness + text_offset.0 - text_padding,
        pos.1 - border_thickness + text_offset.1 - text_padding,
        text_size.0 as u32 + border_thickness as u32 * 2 + text_padding as u32 * 2,
        text_size.1 as u32 + border_thickness as u32 * 2 + text_padding as u32 * 2,
    )
    .unwrap();
    ctx.draw_rect(border_color, pos.0, pos.1, stats_size.0, stats_size.1)
        .unwrap();

    ctx.draw_rect(
        (0, 0, 0),
        pos.0 + border_thickness,
        pos.1 + border_thickness,
        stats_size.0 - border_thickness as u32 * 2,
        stats_size.1 - border_thickness as u32 * 2,
    )
    .unwrap();
    ctx.draw_rect(
        (0, 0, 0),
        pos.0 + text_offset.0 - text_padding,
        pos.1 + text_offset.1 - text_padding,
        text_size.0 as u32 + text_padding as u32 * 2,
        text_size.1 as u32 + text_padding as u32 * 2,
    )
    .unwrap();
}

fn draw_match_stats(ctx: &mut Context, match_hero: MatchHero) {
    let lives = match_hero.lives.to_string();
    let font = ctx.load_font("textures/ttf/OpenSans.ttf", 24).unwrap();
    let text = ctx.render_text(font, lives, (255, 255, 255)).unwrap();
    let stats_size = (50, 50);
    let border_thickness = 2;
    let border_color = (255, 255, 255);

    let hero_sprite = {
        let path = crate::hero_info::HeroInfo::from(match_hero.hero.hero_type).texture_path;
        ctx.load_texture(path).unwrap()
    };

    let (pos, text_offset) = match match_hero.kind {
        PlayerKind::Left => (
            (0, 0),
            (
                stats_size.0 as i32 - stats_size.0 as i32 / 5,
                stats_size.0 as i32 / 8 + stats_size.0 as i32 / 2,
            ),
        ),
        PlayerKind::Right => (
            (1280 - stats_size.0 as i32, 0),
            (
                -text.size.0 + stats_size.0 as i32 / 5,
                stats_size.0 as i32 / 8 + stats_size.0 as i32 / 2,
            ),
        ),
    };

    draw_match_stats_background(
        ctx,
        border_color,
        border_thickness,
        pos,
        text_offset,
        text.size,
        stats_size,
    );

    ctx.draw_texture_sized(
        hero_sprite,
        pos.0 + border_thickness,
        pos.1 + border_thickness,
        stats_size.0 - border_thickness as u32 * 2,
        stats_size.1 - border_thickness as u32 * 2,
    )
    .unwrap();
    ctx.draw_texture(
        text.texture,
        pos.0 + text_offset.0,
        pos.1 - border_thickness + text_offset.1,
    )
    .unwrap();
}

pub struct KnockoffSystem(pub u64);
impl System for KnockoffSystem {
    fn on_add(&self, ctx: &mut Context) -> Result<(), Error> {
        spawn!(ctx, TrashTalkOffset(0.0));
        Ok(())
    }
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        let max_offset_from_screen = 200.0;
        for id in query!(ctx, PlayerMovement, RigidBody, MatchHero).clone() {
            let rigid_body = ctx.select::<RigidBody>(id).clone();
            let match_hero = ctx.select::<MatchHero>(id).clone();
            draw_match_stats(ctx, match_hero);
            if rigid_body.pos.0 + rigid_body.rect.0 < -max_offset_from_screen
                || rigid_body.pos.0 > 1280.0 + max_offset_from_screen
                || rigid_body.pos.1 + rigid_body.rect.1 < -max_offset_from_screen
                || rigid_body.pos.1 > 720.0 + max_offset_from_screen
            {
                let loser_id = id;
                let stats = ctx.select::<MatchHero>(loser_id);
                if stats.lives > 0 {
                    stats.lives -= 1;
                };
                if stats.lives <= 0 {
                    win_condition(ctx, loser_id);
                    continue;
                }
                let rigid_body = ctx.select::<RigidBody>(loser_id);
                rigid_body.pos = ((1280.0 - rigid_body.rect.0) / 2.0, 100.0);
                rigid_body.vel = (0.0, 0.0);
            }
            let trash_talk_offset = ctx.select_one::<TrashTalkOffset>();
            trash_talk_offset.0 += delta;
        }
        Ok(())
    }
    fn on_remove(&self, ctx: &mut Context) -> Result<(), Error> {
        for id in query!(ctx, TrashTalkOffset) {
            ctx.despawn(id);
        }
        Ok(())
    }
}
