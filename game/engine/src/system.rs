use super::{context::Context, Error};

pub trait System
where
    Self: 'static,
{
    fn on_add(&self, _ctx: &mut Context) -> Result<(), Error> {
        Ok(())
    }
    fn on_update(&self, _ctx: &mut Context, _delta: f64) -> Result<(), Error> {
        Ok(())
    }
    fn on_remove(&self, _ctx: &mut Context) -> Result<(), Error> {
        Ok(())
    }
    fn inner_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }
}
