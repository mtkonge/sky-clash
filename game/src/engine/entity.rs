use super::{id::Id, Component};

pub struct Entity(pub Id, pub Vec<Box<dyn Component>>);
