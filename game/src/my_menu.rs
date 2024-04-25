use crate::engine::System;
use crate::engine::{self};
use crate::ui2;

pub struct MyMenu {}

pub struct MyMenuSystem(pub u64);
impl System for MyMenuSystem {
    fn on_add(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui2::builder::constructors::*;
        let _dom = ui2::Dom::new(Vert([
            Text("hello"),
            Hori([Text("world").with_id(12), Text(":3")]),
        ]));

        Ok(())
    }

    fn on_update(&self, _ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}
