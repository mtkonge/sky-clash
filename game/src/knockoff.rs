use engine::{query, rigid_body::RigidBody, Context, Error, System};

use crate::{hurtbox::Player, player_movement::PlayerMovement};

pub struct KnockoffSystem(pub u64);
impl System for KnockoffSystem {
    fn on_update(&self, ctx: &mut Context, _delta: f64) -> Result<(), Error> {
        let max_offset_from_screen = 200.0;
        for id in query!(ctx, PlayerMovement, RigidBody, Player).clone() {
            let rigid_body = ctx.select::<RigidBody>(id).clone();
            if rigid_body.pos.0 + rigid_body.rect.0 < -max_offset_from_screen
                || rigid_body.pos.0 > 1280.0 + max_offset_from_screen
                || rigid_body.pos.1 + rigid_body.rect.1 < -max_offset_from_screen
                || rigid_body.pos.1 > 720.0 + max_offset_from_screen
            {
                let loser_id = id;
                let stats = ctx.select::<Player>(loser_id);
                if stats.lives > 0 {
                    stats.knockback_modifier = 0.0;
                    stats.lives -= 1;
                };
                if stats.lives <= 0 {
                    continue;
                };
                let rigid_body = ctx.select::<RigidBody>(loser_id);
                rigid_body.pos = ((1280.0 - rigid_body.rect.0) / 2.0, 100.0);
                rigid_body.vel = (0.0, 0.0);
            }
        }
        Ok(())
    }
}
