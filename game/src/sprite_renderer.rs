use engine::{query, rigid_body::RigidBody, Component, System};

#[derive(Component)]
pub struct Sprite {
    pub sprite: engine::Texture,
}

pub struct SpriteRenderer(pub u64);
impl System for SpriteRenderer {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, Sprite, RigidBody) {
            let body = ctx.select::<RigidBody>(id).clone();
            let sprite = ctx.select::<Sprite>(id).sprite;

            ctx.draw_texture(sprite, body.pos.0 as i32, body.pos.1 as i32)?;
        }
        Ok(())
    }
}
