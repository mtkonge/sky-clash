use engine::{query, rigid_body::RigidBody, spawn, Context, Error, System};

use crate::{hud::TrashTalk, player::Player, player_movement::PlayerMovement};

pub struct KnockoffSystem(pub u64);
impl System for KnockoffSystem {
    fn on_update(&self, ctx: &mut Context, _delta: f64) -> Result<(), Error> {
        let max_offset_from_screen = 200.0;
        for id in query!(ctx, PlayerMovement, RigidBody, Player).clone() {
            let rigid_body = ctx.select::<RigidBody>(id).clone();
            if body_outside_area(rigid_body, max_offset_from_screen) {
                let loser_id = id;
                let player = ctx.select::<Player>(loser_id);
                if player.is_alive() {
                    player.knockback_modifier = 0.0;
                    player.lives -= 1;
                };
                let player_is_dead = player.is_dead();
                if player_is_dead {
                    let loser_hero_kind = player.hero.kind.clone();
                    let winner_hero_kind = ctx.select_one::<Player>().hero.kind.clone();
                    ctx.despawn(loser_id);
                    spawn!(ctx, TrashTalk::new(winner_hero_kind, loser_hero_kind));
                }
                let rigid_body = ctx.select::<RigidBody>(loser_id);
                rigid_body.pos = ((1280.0 - rigid_body.rect.0) / 2.0, 100.0);
                rigid_body.vel = (0.0, 0.0);
            }
        }
        Ok(())
    }
}

fn body_outside_area(rigid_body: RigidBody, max_offset_from_screen: f64) -> bool {
    rigid_body.pos.0 + rigid_body.rect.0 < -max_offset_from_screen
        || rigid_body.pos.0 > 1280.0 + max_offset_from_screen
        || rigid_body.pos.1 + rigid_body.rect.1 < -max_offset_from_screen
        || rigid_body.pos.1 > 720.0 + max_offset_from_screen
}
