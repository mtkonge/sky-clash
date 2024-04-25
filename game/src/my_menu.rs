use std::rc::Rc;
use std::sync::Mutex;

use crate::engine::{self};
use crate::engine::{Component, System};
use crate::{query, spawn, ui2};

#[derive(Component, Clone)]
pub struct MyMenu {
    dom: Rc<Mutex<ui2::Dom>>,
}

pub struct MyMenuSystem(pub u64);
impl System for MyMenuSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui2::builder::constructors::*;

        let mut dom = ui2::Dom::new(Vert([
            Text("hello"),
            Hori([Text("world").with_id(12), Text(":3").with_id(34)]),
        ]));

        dom.add_event_handler(12, |dom, _ctx, _node_id| {
            let Some(other) = dom.select(34) else {return;};
            match other.kind {
                ui2::Kind::Text(ref mut text) => {
                    *text = "some thing else".to_string();
                }
                _ => unreachable!(),
            }
        });

        spawn!(
            ctx,
            MyMenu {
                dom: Rc::new(Mutex::new(dom))
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, MyMenu) {
            let my_menu = ctx.entity_component::<MyMenu>(id).clone();
            my_menu.dom.lock().unwrap().update(ctx);
        }
        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}
