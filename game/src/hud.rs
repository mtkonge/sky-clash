use engine::{query, Component, Context, DrawTextureOpts, Error, System, V2};
use shared::HeroKind;

use crate::player::{Player, PlayerKind};

pub struct HudSystem(pub u64);

impl System for HudSystem {
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        for id in query!(ctx, Player).clone() {
            let player = ctx.select::<Player>(id).clone();
            draw_hud(ctx, &player);
        }
        for id in query!(ctx, TrashTalk).clone() {
            let trash_talk = ctx.select::<TrashTalk>(id);
            trash_talk.text_cycle_clock += delta;
            let trash_talk = ctx.select::<TrashTalk>(id).clone();
            trash_talk.draw(ctx);
        }
        Ok(())
    }
}

#[derive(Clone, Component)]
pub struct TrashTalk {
    pub winner: HeroKind,
    pub loser: HeroKind,
    pub text_cycle_clock: f64,
}

impl TrashTalk {
    pub fn new(winner: HeroKind, loser: HeroKind) -> Self {
        Self {
            winner,
            loser,
            text_cycle_clock: 0.0,
        }
    }

    fn draw(&self, ctx: &mut Context) {
        let trash_talk = self.loser_text();
        let font = ctx.load_font("textures/ttf/OpenSans.ttf", 48).unwrap();
        let text = ctx.render_text(font, &trash_talk, (255, 255, 255)).unwrap();
        ctx.draw_texture(
            text.texture,
            V2::new((1280.0 - text.size.x) / 2.0, 100.0),
            DrawTextureOpts::new(),
        )
        .unwrap();
    }

    fn loser_text(&self) -> String {
        let (winner, loser) = (&self.winner, &self.loser);
        let amount_of_messages = 9;
        let counter = self.text_cycle_clock as u64 % amount_of_messages;
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
}

fn linear_interpolation(current: u8, next: u8, percentage: f64) -> u8 {
    (f64::from(current) * (1.0 - percentage) + f64::from(next) * percentage) as u8
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

pub fn player_damage_color(damage_taken: f64) -> (u8, u8, u8) {
    let damage_taken_per_step = 75.0;
    let transition_alpha = damage_taken % damage_taken_per_step;
    let colors = [
        (255, 255, 255),
        (255, 255, 0),
        (255, 127, 0),
        (255, 0, 0),
        (127, 0, 0),
        (30, 30, 30),
    ];
    let max_idx = colors.len() - 1;
    let idx = ((damage_taken - transition_alpha) / damage_taken_per_step) as usize;
    let current = std::cmp::min(max_idx, idx);
    let next = std::cmp::min(max_idx, idx + 1);
    let transition_percentage = (damage_taken % damage_taken_per_step) / damage_taken_per_step;
    merge_colors(colors[current], colors[next], transition_percentage)
}

fn draw_player_background(
    ctx: &mut Context,
    player: &Player,
    border_color: (u8, u8, u8),
    border_pos: V2,
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

    ctx.draw_texture(border, border_pos, DrawTextureOpts::new())
        .unwrap();
    ctx.draw_texture(
        border_outline,
        border_pos,
        DrawTextureOpts::new().color_mod(border_color),
    )
    .unwrap();
}

fn draw_player_stats(
    ctx: &mut Context,
    player: &Player,
    avatar_pos: V2,
    avatar_size: V2,
    text_pos: V2,
) {
    let hero_sprite = {
        let path = crate::hero_info::HeroInfo::from(&player.hero.kind).texture_path;
        ctx.load_texture(path).unwrap()
    };

    let font = ctx.load_font("textures/ttf/OpenSans.ttf", 24).unwrap();
    let lives = player.lives.to_string();
    let lives = ctx.render_text(font, lives, (255, 255, 255)).unwrap();

    ctx.draw_texture(
        hero_sprite,
        avatar_pos,
        DrawTextureOpts::new().size(avatar_size),
    )
    .unwrap();
    ctx.draw_texture(lives.texture, text_pos, DrawTextureOpts::new())
        .unwrap();
}

fn draw_hud(ctx: &mut Context, player: &Player) {
    let stats_size = V2::new(100.0, 88.0);
    let border_color = player_damage_color(player.damage_taken);

    let border_pos = match player.kind {
        PlayerKind::Left => V2::new(8.0, 8.0),
        PlayerKind::Right => V2::new(1280.0 - stats_size.x - 8.0, 8.0),
    };

    draw_player_background(ctx, player, border_color, border_pos);

    let avatar_pos = match player.kind {
        PlayerKind::Left => border_pos + V2::new(8.0, 8.0),
        PlayerKind::Right => border_pos + V2::new(28.0, 8.0),
    };

    let avatar_size = V2::new(64.0, 64.0);

    let text_pos = match player.kind {
        PlayerKind::Left => border_pos + V2::new(78.0 + 1.0, 58.0 - 6.0),
        PlayerKind::Right => border_pos + V2::new(6.0 + 1.0, 58.0 - 6.0),
    };

    draw_player_stats(ctx, player, avatar_pos, avatar_size, text_pos);
}
