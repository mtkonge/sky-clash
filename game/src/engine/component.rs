use std::any::{Any, TypeId};

pub trait Component {
    fn inner_type_id(&self) -> TypeId;
    fn as_any(&mut self) -> &mut dyn Any;
}
