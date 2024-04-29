use std::rc::Rc;
use std::sync::Mutex;

use crate::ui2::components::Button;
use crate::ui2::{self, BoxedNode, Dom, NodeId};
use engine::{query, spawn};
use engine::{Component, System};

#[derive(Component, Clone)]
pub struct HeroCreator {
    dom: Rc<Mutex<ui2::Dom>>,
}

fn percentage<S: Into<String>>(text: S, filled_steps: usize, unfilled_steps: usize) -> BoxedNode {
    use ui2::constructors::*;

    let max_steps = filled_steps + unfilled_steps;
    let middle = max_steps / 2;
    let mut children: Vec<_> = (0..max_steps)
        .map(|i| {
            if i < filled_steps {
                Text("|").with_color((255, 255, 255))
            } else {
                Text("|").with_color((127, 127, 127))
            }
        })
        .collect();
    children.insert(middle, Text(text));
    children.insert(middle + 1, Text(" "));
    children.insert(middle, Text(" "));
    Hori(children)
        .with_padding(4)
        .with_border_color((255, 255, 255))
}

pub struct HeroCreatorSystem(pub u64);
impl System for HeroCreatorSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui2::constructors::*;

        let mut dom = ui2::Dom::new(
            Hori([Vert([Hori([
                Text("-").with_on_click(1),
                Text(" "),
                percentage("Kawaii! T~T UwU", 4, 8),
                Text(" "),
                Text("+").with_on_click(2),
            ])])
            .with_width(1280)])
            .with_width(1280)
            .with_height(720)
            .with_background_color((0, 0, 0)),
        );

        dom.add_event_handler(
            1,
            |dom: &mut Dom, ctx: &mut engine::Context, _id: NodeId| {},
        );

        dom.add_event_handler(
            2,
            |dom: &mut Dom, ctx: &mut engine::Context, _id: NodeId| {},
        );

        spawn!(
            ctx,
            HeroCreator {
                dom: Rc::new(Mutex::new(dom))
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, HeroCreator) {
            let my_menu = ctx.entity_component::<HeroCreator>(id).clone();
            my_menu.dom.lock().unwrap().update(ctx);
        }
        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}
