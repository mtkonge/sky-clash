use engine::{query, spawn, Component, Context, Error, System};

use crate::hurtbox::{Player, PlayerKind};

fn player_died_text(loser: &shared::HeroKind, winner: &shared::HeroKind, counter: f64) -> String {
    let amount_of_messages = 9;
    let counter = counter as u8 % amount_of_messages;
    match counter {
        0 => format!("looks like {loser} has skill issues"),
        1 => format!("{loser} was not Him"),
        2 => format!("bro lost to a {winner}"),
        3 => format!("dying to a {winner} in 2024 is crazy"),
        4 => format!("{loser} is so loserpilled"),
        5 => format!("bro lost to a {winner} before Gta VI"),
        6 => format!("{loser} is losermaxxing"),
        7 => format!("in loser town everybody knows {loser}"),
        8 => format!("dying to a {winner} won't pay the bills"),
        _ => unreachable!(),
    }
}

fn linear_interpolation(current: u8, next: u8, percentage: f64) -> u8 {
    (current as f64 * (1.0 - percentage) + next as f64 * percentage) as u8
}

fn merge_colors(
    current: (u8, u8, u8),
    next: (u8, u8, u8),
    transition_percentage: f64,
) -> (u8, u8, u8) {
    (
        linear_interpolation(current.0, next.0, transition_percentage),
        linear_interpolation(current.1, next.1, transition_percentage),
        linear_interpolation(current.2, next.2, transition_percentage),
    )
}

fn player_damage_color(damage_taken: f64) -> (u8, u8, u8) {
    let transition_alpha = damage_taken % 2.5;
    let colors = [
        (255, 255, 255),
        (255, 255, 0),
        (255, 127, 0),
        (255, 0, 0),
        (127, 0, 0),
        (30, 30, 30),
    ];
    let max_idx = colors.len() - 1;
    let idx = ((damage_taken - transition_alpha) / 2.5) as usize;
    let current = std::cmp::min(max_idx, idx);
    let next = std::cmp::min(max_idx, idx + 1);
    let transition_percentage = (damage_taken % 2.5) / 2.5;
    merge_colors(colors[current], colors[next], transition_percentage)
}

#[derive(Component, Clone)]
pub struct TrashTalkOffset(f64);

fn draw_trash_talk(ctx: &mut Context, loser_id: engine::Id) {
    let winner = 'winner: {
        for winner_id in query!(ctx, Player) {
            if winner_id == loser_id {
                continue;
            }
            break 'winner ctx.select::<Player>(winner_id).hero.hero_type.clone();
        }
        unreachable!("other player somehow despawned");
    };
    let trash_talk_offset = ctx.select_one::<TrashTalkOffset>().0;
    let loser = &ctx.select::<Player>(loser_id).hero.hero_type;
    let trash_talk = player_died_text(loser, &winner, trash_talk_offset);
    let font = ctx.load_font("textures/ttf/OpenSans.ttf", 48).unwrap();
    let text = ctx.render_text(font, &trash_talk, (255, 255, 255)).unwrap();
    ctx.draw_texture(text.texture, (1280 - text.size.0) / 2, 100)
        .unwrap();
}

fn draw_player_background(
    ctx: &mut Context,
    player: &Player,
    border_color: (u8, u8, u8),
    border_pos: (i32, i32),
) {
    let border_path = match player.kind {
        PlayerKind::Left => "textures/stats_left.png",
        PlayerKind::Right => "textures/stats_right.png",
    };
    let border_outline_path = match player.kind {
        PlayerKind::Left => "textures/stats_left_outline.png",
        PlayerKind::Right => "textures/stats_right_outline.png",
    };
    let border = ctx.load_texture(border_path).unwrap();
    let border_outline = ctx.load_texture(border_outline_path).unwrap();

    ctx.draw_texture(border, border_pos.0, border_pos.1)
        .unwrap();
    ctx.draw_texture_with_color_mod(border_outline, border_pos.0, border_pos.1, border_color)
        .unwrap();
}

fn draw_player_stats(
    ctx: &mut Context,
    player: &Player,
    avatar_pos: (i32, i32),
    avatar_size: (u32, u32),
    text_pos: (i32, i32),
) {
    let hero_sprite = {
        let path = crate::hero_info::HeroInfo::from(&player.hero.hero_type).texture_path;
        ctx.load_texture(path).unwrap()
    };

    let font = ctx.load_font("textures/ttf/OpenSans.ttf", 24).unwrap();
    let lives = player.lives.to_string();
    let lives = ctx.render_text(font, lives, (255, 255, 255)).unwrap();

    ctx.draw_texture_sized(
        hero_sprite,
        avatar_pos.0,
        avatar_pos.1,
        avatar_size.0,
        avatar_size.1,
    )
    .unwrap();
    ctx.draw_texture(lives.texture, text_pos.0, text_pos.1)
        .unwrap();
}

fn draw_hud(ctx: &mut Context, player: &Player) {
    let stats_size = (100, 88);
    let border_color = player_damage_color(player.knockback_modifier);

    let border_pos = match player.kind {
        PlayerKind::Left => (8, 8),
        PlayerKind::Right => (1280 - stats_size.0 - 8, 8),
    };

    draw_player_background(ctx, &player, border_color, border_pos);

    let avatar_pos = match player.kind {
        PlayerKind::Left => (border_pos.0 + 8, border_pos.1 + 8),
        PlayerKind::Right => (border_pos.0 + 28, border_pos.1 + 8),
    };

    let avatar_size = (64, 64);

    let text_pos = match player.kind {
        PlayerKind::Left => (border_pos.0 + 78 + 1, border_pos.1 + 58 - 6),
        PlayerKind::Right => (border_pos.0 + 6 + 1, border_pos.1 + 58 - 6),
    };

    draw_player_stats(ctx, &player, avatar_pos, avatar_size, text_pos)
}

pub struct HudSystem(pub u64);

impl System for HudSystem {
    fn on_add(&self, ctx: &mut Context) -> Result<(), Error> {
        spawn!(ctx, TrashTalkOffset(0.0));
        Ok(())
    }
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        for id in query!(ctx, Player).clone() {
            let player = ctx.select::<Player>(id).clone();
            draw_hud(ctx, &player);
            if player.lives <= 0 {
                draw_trash_talk(ctx, id);
                continue;
            }
        }
        let trash_talk_offset = ctx.select_one::<TrashTalkOffset>();
        trash_talk_offset.0 += delta;
        Ok(())
    }
    fn on_remove(&self, ctx: &mut Context) -> Result<(), Error> {
        for id in query!(ctx, TrashTalkOffset) {
            ctx.despawn(id);
        }
        Ok(())
    }
}
