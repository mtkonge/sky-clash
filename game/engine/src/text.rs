use crate::V2;

use super::Texture;

#[derive(Clone, Copy)]
#[must_use]
pub struct Text {
    pub texture: Texture,
    pub size: V2,
}
