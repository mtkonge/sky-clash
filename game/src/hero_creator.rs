use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Mutex;

use crate::comms::{CreateHeroParams, HeroOrRfid};
use crate::hero_info::{HeroInfo, HeroType};
use crate::ui;
use crate::ui::components::Button;
use crate::Comms;
use engine::{query, query_one, spawn};
use engine::{Component, System};

pub fn change_text_node_content<S: Into<String>>(node: Option<&mut ui::Node>, new_text: S) {
    let Some(ui::Node {
        kind: ui::Kind::Text { ref mut text, .. },
        ..
    }) = node
    else {
        return;
    };
    *text = new_text.into()
}

pub fn change_image_node_content<P: Into<PathBuf>>(node: Option<&mut ui::Node>, new_path: P) {
    let Some(ui::Node {
        kind: ui::Kind::Image(ref mut image),
        ..
    }) = node
    else {
        return;
    };
    *image = new_path.into()
}

#[derive(Component, Clone)]
pub struct HeroCreator {
    dom: Rc<Mutex<ui::Dom>>,
    strength_bar: Rc<Mutex<ui::components::ProgressBar>>,
    defence_bar: Rc<Mutex<ui::components::ProgressBar>>,
    agility_bar: Rc<Mutex<ui::components::ProgressBar>>,
}

#[derive(Component, Clone)]
pub struct Rfid(Option<String>);

pub struct HeroCreatorSystem(pub u64);
impl System for HeroCreatorSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui::constructors::*;

        #[repr(u64)]
        enum NodeId {
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

        let strength_bar = ui::components::ProgressBar::new("Strength", 24, 100);
        let agility_bar = ui::components::ProgressBar::new("Agility", 24, 300);
        let defence_bar = ui::components::ProgressBar::new("Defence", 24, 200);

        spawn!(ctx, Rfid(None));

        let mut dom = ui::Dom::new(
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
                        Text("Available points: 0").with_id(NodeId::AvailablePoints),
                        strength_bar.build(),
                        agility_bar.build(),
                        defence_bar.build(),
                        Hori([ui::components::Button("Confirm")]),
                        Rect().with_height(720 / 2 - 100),
                    ]),
                ]),
                Vert([
                    Text("Unknown hero").with_padding(5),
                    Text("Please select which hero this is").with_padding(5),
                    Hori([
                        Button("Centrist")
                            .with_background_color((100, 100, 100))
                            .with_padding(5)
                            .on_click(10),
                        Button("Speed")
                            .with_background_color((100, 100, 100))
                            .with_padding(5)
                            .on_click(11),
                        Button("Strong")
                            .with_background_color((100, 100, 100))
                            .with_padding(5)
                            .on_click(12),
                        Button("Tankie")
                            .with_background_color((100, 100, 100))
                            .with_padding(5)
                            .on_click(13),
                    ]),
                ])
                .with_id(NodeId::HeroSelectPopup)
                .visible(false)
                .with_border_thickness(2)
                .with_padding(5),
                Vert([
                    Text("Error").with_id(NodeId::ErrorText).with_padding(5),
                    Button("Ok")
                        .with_background_color((100, 100, 100))
                        .with_padding(5)
                        .on_click(1),
                ])
                .with_id(NodeId::ErrorPopup)
                .visible(false)
                .with_border_thickness(2)
                .with_padding(5),
            ])
            .with_width(1280)
            .with_height(720)
            .with_background_color((50, 50, 50)),
        );
        strength_bar.add_event_handlers(&mut dom);
        agility_bar.add_event_handlers(&mut dom);
        defence_bar.add_event_handlers(&mut dom);

        for id in query!(ctx, Comms) {
            let comms = ctx.entity_component::<Comms>(id);
            comms.req_sender.send(crate::CommReq::BoardStatus).unwrap();
        }

        dom.add_event_handler(1, |dom, _ctx, _node_id| {
            dom.select_mut(5).unwrap().set_visible(false);
        });

        for (id, hero_type) in [
            (10, HeroType::Centrist),
            (11, HeroType::Speed),
            (12, HeroType::Strong),
            (13, HeroType::Tankie),
        ] {
            dom.add_event_handler(id, move |dom, ctx, _node_id| {
                let hero_type = hero_type.clone();
                let Rfid(Some(rfid)) = ctx.entity_component::<Rfid>(query_one!(ctx, Rfid)).clone()
                else {
                    return;
                };
                let comms = ctx.entity_component::<Comms>(query_one!(ctx, Comms));
                match comms
                    .req_sender
                    .send(crate::CommReq::CreateHero(CreateHeroParams {
                        rfid,
                        hero_type: hero_type.clone(),
                    })) {
                    Ok(_) => {
                        change_image_node_content(
                            dom.select_mut(2),
                            HeroInfo::from(hero_type).texture_path,
                        );
                        dom.select_mut(4).unwrap().set_visible(false);
                    }
                    Err(_) => println!("Nooooooo :("),
                }
            });
        }

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
        let comms = ctx.entity_component::<Comms>(query_one!(ctx, Comms));
        comms.req_sender.send(crate::CommReq::BoardStatus).unwrap();

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
                    Ok(HeroOrRfid::Hero(hero)) => {
                        let rfid = ctx.entity_component::<Rfid>(query_one!(ctx, Rfid));
                        rfid.0.replace(hero.rfid);

                        change_text_node_content(
                            dom.select_mut(3),
                            format!("Available points: {}", hero.level * 3),
                        );
                    }

                    Ok(HeroOrRfid::Rfid(rfid_value)) => {
                        let rfid = ctx.entity_component::<Rfid>(query_one!(ctx, Rfid));
                        rfid.0.replace(rfid_value);

                        dom.select_mut(4).unwrap().set_visible(true);
                    }
                    Err(err) => {
                        dom.select_mut(5).unwrap().set_visible(true);
                        change_text_node_content(
                            dom.select_mut(6),
                            format!("an error occurred: {err}"),
                        );
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
