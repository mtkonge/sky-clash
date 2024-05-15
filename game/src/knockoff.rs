use engine::{query, rigid_body::RigidBody, spawn, Component, Context, Error, System};

use crate::{hurtbox::MatchHero, player_movement::PlayerMovement};

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

pub struct KnockoffSystem(pub u64);
impl System for KnockoffSystem {
    fn on_add(&self, ctx: &mut Context) -> Result<(), Error> {
        spawn!(ctx, TrashTalkOffset(0.0));
        Ok(())
    }
    fn on_update(&self, ctx: &mut Context, delta: f64) -> Result<(), Error> {
        let max_offset_from_screen = 200.0;
        for loser_id in query!(ctx, PlayerMovement, RigidBody, MatchHero).clone() {
            let rigid_body = ctx.select::<RigidBody>(loser_id).clone();
            if rigid_body.pos.0 + rigid_body.rect.0 < -max_offset_from_screen
                || rigid_body.pos.0 > 1280.0 + max_offset_from_screen
                || rigid_body.pos.1 + rigid_body.rect.1 < -max_offset_from_screen
                || rigid_body.pos.1 > 720.0 + max_offset_from_screen
            {
                let stats = ctx.select::<MatchHero>(loser_id);
                if stats.lives > 0 {
                    stats.lives -= 1;
                };
                if stats.lives <= 0 {
                    let winner = 'winner: {
                        for winner_id in query!(ctx, PlayerMovement, RigidBody, MatchHero) {
                            if winner_id == loser_id {
                                continue;
                            }
                            break 'winner ctx
                                .select::<MatchHero>(winner_id)
                                .hero
                                .hero_type
                                .clone();
                        }
                        unreachable!("other player somehow despawned");
                    };
                    let trash_talk_offset = ctx.select_one::<TrashTalkOffset>().0;
                    let loser = &ctx.select::<MatchHero>(loser_id).hero.hero_type;
                    let trash_talk = player_died_text(loser, &winner, trash_talk_offset);
                    let font = ctx.load_font("textures/ttf/OpenSans.ttf", 48).unwrap();
                    let text = ctx.render_text(font, &trash_talk, (255, 255, 255)).unwrap();
                    let (text_width, _) = ctx.text_size(font, &trash_talk).unwrap();
                    ctx.draw_texture(text.texture, (1280 - text_width as i32) / 2, 100)
                        .unwrap();
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
