use engine::{query, rigid_body::RigidBody, Component, DrawTextureOpts, System, V2};

#[derive(Component, Debug, Clone)]
pub struct Sprite {
    pub offset: V2,
    pub size: Option<V2>,
    pub texture: engine::Texture,
    pub layer: i32,
    pub opacity: Option<f64>,
}

impl Sprite {
    pub fn new(texture: engine::Texture) -> Self {
        Self {
            texture,
            layer: 0,
            offset: V2::new(0.0, 0.0),
            size: None,
            opacity: None,
        }
    }

    pub fn layer(self, layer: i32) -> Self {
        Self { layer, ..self }
    }

    pub fn size(self, size: V2) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    pub fn offset(self, offset: V2) -> Self {
        Self { offset, ..self }
    }

    pub fn set_opacity(&mut self, opacity: f64) {
        self.opacity = Some(opacity);
    }
}

pub struct SpriteRenderer(pub u64);
impl System for SpriteRenderer {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        let mut sprites = Vec::<(Sprite, V2, V2)>::new();
        for id in query!(ctx, RigidBody, Sprite) {
            let body = ctx.select::<RigidBody>(id).clone();
            let sprite = ctx.select::<Sprite>(id).clone();

            sprites.push((sprite, body.pos, body.size));
        }
        sprites.sort_by(|(a, _, _), (b, _, _)| b.layer.cmp(&a.layer));
        for (sprite, pos, body_size) in sprites {
            let size = sprite.size.unwrap_or(body_size);
            let opacity = sprite.opacity.unwrap_or(1.0);
            ctx.draw_texture(
                sprite.texture,
                pos + sprite.offset,
                DrawTextureOpts::new().size(size).opacity(opacity),
            )?;
        }
        Ok(())
    }
}
