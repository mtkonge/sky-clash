use std::any::{Any, TypeId};

pub trait Component
where
    Self: 'static,
{
    fn inner_type_id(&self) -> TypeId {
        std::any::TypeId::of::<Self>()
    }
    fn as_any(&mut self) -> &mut dyn Any;
}
