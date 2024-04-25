use crate::engine::System;
use crate::{query, spawn};

pub struct MyMenu(pub u64);
impl System for MyMenu {
    fn on_add(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
        Ok(())
    }
    fn on_update(
        &self,
        ctx: &mut crate::engine::Context,
        _delta: f64,
    ) -> Result<(), crate::engine::Error> {
        Ok(())
    }
    fn on_remove(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
        Ok(())
    }
}
