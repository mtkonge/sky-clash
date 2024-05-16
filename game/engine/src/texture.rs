use super::id::Id;

#[derive(Clone, Copy, Debug)]
pub struct Texture(pub Id);

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TextTextureKey(pub Id, pub String, pub (u8, u8, u8));
