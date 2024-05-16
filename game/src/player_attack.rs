use engine::{query, rigid_body::RigidBody, spawn, Collider, Component, System};

use crate::{
    hurtbox::{HurtDirection, Hurtbox},
    key_set::KeySet,
    sprite_renderer::Sprite,
};

#[derive(Component, Clone)]
pub struct PlayerAttack {
    pub key_set: KeySet,
}

pub struct PlayerAttackSystem(pub u64);
impl System for PlayerAttackSystem {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, PlayerAttack, RigidBody, Collider) {
            let key_set = ctx.select::<PlayerAttack>(id).clone().key_set;
            let right_pressed = ctx.key_pressed(key_set.right());
            let left_pressed = ctx.key_pressed(key_set.left());
            let down_pressed = ctx.key_pressed(key_set.down());
            let light_attack_pressed = ctx.key_pressed(key_set.light_attack());
            let body = ctx.select::<RigidBody>(id).clone();
            let hurtbox_texture = ctx.load_texture("textures/nuh-uh.png").unwrap();
            if !light_attack_pressed {
                continue;
            }
            if down_pressed {
                println!("down attack");
                spawn!(
                    ctx,
                    Sprite {
                        sprite: hurtbox_texture
                    },
                    RigidBody {
                        pos: (body.pos.0, body.pos.1 + body.rect.1),
                        rect: (128.0, 128.0),
                        ..Default::default()
                    },
                    Hurtbox {
                        direction: HurtDirection::Down,
                        power: 20.0,
                        owner: Some(id),
                        duration: 1.0,
                        stun_time: Some(10.0),
                        ..Default::default()
                    },
                );
            } else if left_pressed && !right_pressed {
                println!("left attack")
            } else if right_pressed && !left_pressed {
                println!("right attack")
            } else {
                println!("neutral attack")
            }
        }

        Ok(())
    }
}
