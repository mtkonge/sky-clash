use std::rc::Rc;
use std::sync::Mutex;

use crate::ui2;
use crate::Comms;
use engine::{query, query_one, spawn};
use engine::{Component, System};

pub fn change_text_node_content<S: Into<String>>(node: Option<&mut ui2::Node>, new_text: S) {
    let Some(ui2::Node {
        kind: ui2::Kind::Text { ref mut text, .. },
        ..
    }) = node
    else {
        return;
    };
    *text = new_text.into()
}

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

        #[repr(u64)]
        enum NodeId {
            BoardRetrieverText = 0,
            HeroTypeText = 1,
            HeroImage = 2,
            AvailablePoints = 3,
            HeroSelectPopup = 4,
            ErrorPopup = 5,
            ErrorText = 6,
        }

        impl From<NodeId> for u64 {
            fn from(value: NodeId) -> Self {
                value as u64
            }
        }

        let strength_bar = ui2::components::ProgressBar::new("Strength", 24, 100);
        let agility_bar = ui2::components::ProgressBar::new("Agility", 24, 300);
        let defence_bar = ui2::components::ProgressBar::new("Defence", 24, 200);

        let mut dom = ui2::Dom::new(
            Stack([
                Hori([
                    Vert([
                        Vert([
                            Image("./textures/player.png")
                                .with_id(NodeId::HeroImage)
                                .with_width(128)
                                .with_height(128),
                            Text("Hero type: boykisser")
                                .with_id(NodeId::HeroTypeText)
                                .with_padding(30),
                            Rect().with_height(720 / 16),
                        ])
                        .with_padding(50)
                        .with_border_thickness(2),
                        Rect().with_height(720 / 4),
                    ]),
                    Rect().with_width(1280 / 4),
                    Vert([
                        Text("Retrieving board").with_id(NodeId::BoardRetrieverText),
                        Text("Available points: 0").with_id(NodeId::AvailablePoints),
                        strength_bar.build(),
                        agility_bar.build(),
                        defence_bar.build(),
                        Hori([ui2::components::Button("Confirm")]),
                        Rect().with_height(720 / 2 - 100),
                    ]),
                ]),
                Vert([
                    Text("Unknown hero").with_padding(5),
                    Text("Please select which hero this is").with_padding(5),
                    Text("Confirm")
                        .with_background_color((100, 100, 100))
                        .with_padding(5)
                        .on_click(0),
                ])
                .with_id(NodeId::HeroSelectPopup)
                .visible(false),
                Vert([
                    Text("Error").with_id(NodeId::ErrorText).with_padding(5),
                    Text("Ok")
                        .with_background_color((100, 100, 100))
                        .with_padding(5)
                        .on_click(1),
                ])
                .with_id(NodeId::ErrorPopup)
                .visible(false),
            ])
            .with_width(1280)
            .with_height(720)
            .with_background_color((0, 0, 0)),
        );
        strength_bar.add_event_handlers(&mut dom);
        agility_bar.add_event_handlers(&mut dom);
        defence_bar.add_event_handlers(&mut dom);

        for id in query!(ctx, Comms) {
            let comms = ctx.entity_component::<Comms>(id);
            comms.req_sender.send(crate::CommReq::BoardStatus).unwrap();
        }

        dom.add_event_handler(0, |dom, _ctx, _node_id| {
            dom.select_mut(4).unwrap().set_visible(false);
        });
        dom.add_event_handler(1, |dom, _ctx, _node_id| {
            dom.select_mut(5).unwrap().set_visible(false);
        });

        spawn!(
            ctx,
            HeroCreator {
                dom: Rc::new(Mutex::new(dom)),
                strength_bar: Rc::new(Mutex::new(strength_bar)),
                agility_bar: Rc::new(Mutex::new(agility_bar)),
                defence_bar: Rc::new(Mutex::new(defence_bar)),
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
            menu.agility_bar.lock().unwrap().update(&mut dom);
            menu.defence_bar.lock().unwrap().update(&mut dom);

            let comms = ctx.entity_component::<Comms>(query_one!(ctx, Comms));
            if let Ok(hero) = comms.board_receiver.try_recv() {
                match hero {
                    Ok(Some(hero)) => {
                        change_text_node_content(
                            dom.select_mut(0),
                            format!("known hero on boawd: {}", hero.rfid),
                        );
                        change_text_node_content(
                            dom.select_mut(3),
                            format!("Available points: {}", hero.level * 3),
                        )
                    }
                    Ok(None) => {
                        change_text_node_content(dom.select_mut(0), "unknown hero on boawd");
                        dom.select_mut(4).unwrap().set_visible(true)
                    }
                    Err(err) => {
                        change_text_node_content(dom.select_mut(0), err.clone());
                        dom.select_mut(5).unwrap().set_visible(true);
                        change_text_node_content(dom.select_mut(6), err);
                    }
                }
            }
        }
        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}
