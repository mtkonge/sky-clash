use crate::engine::System;
use crate::ui2;
use crate::{
    engine::{self},
    query, spawn,
};

pub struct MyMenu {}

pub struct MyMenuSystem(pub u64);
impl System for MyMenuSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui2::builder::constructors::*;
        let dom = ui2::DOM::new(Vert([
            Text("hello"),
            Hori([Text("world").with_id(12), Text(":3")]),
        ]));

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        Ok(())
    }

    fn on_remove(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}
