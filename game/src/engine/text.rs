use super::Texture;

#[derive(Clone, Copy)]
pub struct Text {
    pub texture: Texture,
    pub size: (i32, i32),
}
