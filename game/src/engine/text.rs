use super::Texture;

#[derive(Clone, Copy)]
#[must_use]
pub struct Text {
    pub texture: Texture,
    pub size: (i32, i32),
}
