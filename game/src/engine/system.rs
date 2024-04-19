use super::{context::Context, Error};

pub trait System {
    fn on_add(&self, _ctx: &mut Context) -> Result<(), Error> {
        Ok(())
    }
    fn on_update(&self, _ctx: &mut Context, _delta: f64) -> Result<(), Error> {
        Ok(())
    }
    fn on_remove(&self, _ctx: &mut Context) -> Result<(), Error> {
        Ok(())
    }
}
