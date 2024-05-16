use engine::{query, rigid_body::RigidBody, Component, System};

#[derive(Component, Debug, Clone)]
pub struct Sprite {
    pub texture: engine::Texture,
    pub layer: i32,
}

impl Sprite {
    pub fn new(texture: engine::Texture) -> Self {
        Self { texture, layer: 0 }
    }

    pub fn new_layered(texture: engine::Texture, layer: i32) -> Self {
        Self { texture, layer }
    }
}

pub struct SpriteRenderer(pub u64);
impl System for SpriteRenderer {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        let mut sprites = Vec::<(Sprite, (f64, f64), (f64, f64))>::new();
        for id in query!(ctx, RigidBody, Sprite) {
            let body = ctx.select::<RigidBody>(id).clone();
            let sprite = ctx.select::<Sprite>(id).clone();

            sprites.push((sprite, (body.pos.0, body.pos.1), (body.rect.0, body.rect.1)));
        }
        sprites.sort_by(|(a, _, _), (b, _, _)| b.layer.cmp(&a.layer));
        for (sprite, pos, rect) in sprites {
            ctx.draw_texture_sized(
                sprite.texture,
                pos.0 as i32,
                pos.1 as i32,
                rect.0 as u32,
                rect.1 as u32,
            )?;
        }
        Ok(())
    }
}
