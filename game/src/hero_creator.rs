use std::rc::Rc;
use std::sync::Mutex;

use crate::ui2::{self};
use crate::Comms;
use engine::{query, query_one, spawn};
use engine::{Component, System};

#[derive(Component, Clone)]
pub struct HeroCreator {
    dom: Rc<Mutex<ui2::Dom>>,
    strength_bar: Rc<Mutex<ui2::components::ProgressBar>>,
    defence_bar: Rc<Mutex<ui2::components::ProgressBar>>,
    agility_bar: Rc<Mutex<ui2::components::ProgressBar>>,
}

pub struct HeroCreatorSystem(pub u64);
impl System for HeroCreatorSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui2::constructors::*;

        let strength_bar = ui2::components::ProgressBar::new("Strength", 24, 100);
        let defence_bar = ui2::components::ProgressBar::new("Defence", 24, 200);
        let agility_bar = ui2::components::ProgressBar::new("Strength", 24, 300);

        let mut dom = ui2::Dom::new(
            Stack([Vert([
                Text("Retrieving board").with_id(0),
                strength_bar.build(),
                defence_bar.build(),
                agility_bar.build(),
                Hori([ui2::components::Button("Confirm")]),
            ])])
            .with_width(1280)
            .with_height(720)
            .with_background_color((0, 0, 0)),
        );
        strength_bar.add_event_handlers(&mut dom);
        defence_bar.add_event_handlers(&mut dom);
        agility_bar.add_event_handlers(&mut dom);

        for id in query!(ctx, Comms) {
            let comms = ctx.entity_component::<Comms>(id);
            comms.req_sender.send(crate::CommReq::BoardStatus).unwrap();
        }

        spawn!(
            ctx,
            HeroCreator {
                dom: Rc::new(Mutex::new(dom)),
                strength_bar: Rc::new(Mutex::new(strength_bar)),
                defence_bar: Rc::new(Mutex::new(defence_bar)),
                agility_bar: Rc::new(Mutex::new(agility_bar)),
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, HeroCreator) {
            let menu = ctx.entity_component::<HeroCreator>(id).clone();
            let mut dom = menu.dom.lock().unwrap();
            dom.update(ctx);

            menu.strength_bar.lock().unwrap().update(&mut dom);
            menu.defence_bar.lock().unwrap().update(&mut dom);
            menu.agility_bar.lock().unwrap().update(&mut dom);

            let comms = ctx.entity_component::<Comms>(query_one!(ctx, Comms));
            if let Ok(hero) = comms.board_receiver.try_recv() {
                let Some(ui2::Node {
                    kind: ui2::Kind::Text { text, .. },
                    ..
                }) = dom.select_mut(0)
                else {
                    continue;
                };
                match hero {
                    Ok(Some(hero)) => {
                        *text = format!("known hero on boawd: {}", hero.rfid);
                    }
                    Ok(None) => *text = "unknown hero on boawd".to_string(),
                    Err(err) => *text = err,
                }
            }
        }
        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}
