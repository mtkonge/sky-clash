use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Mutex;

use crate::hero_info::HeroInfo;
use crate::message::HeroOrUnknownRfid;
use crate::ui;
use crate::ui::components::Button;
use crate::Comms;
use engine::{query_one, spawn};
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
pub struct Rfid(Option<HeroOrUnknownRfid>);

#[derive(Component, Clone)]
pub struct SinceLastRequest(f64);

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
        spawn!(ctx, SinceLastRequest(f64::MAX));

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
                        Hori([ui::components::Button("Confirm").on_click(20)]),
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
                Text("Loading...")
                    .with_width(1280)
                    .with_height(720)
                    .with_background_color((50, 50, 50))
                    .with_id(50u64),
            ])
            .with_width(1280)
            .with_height(720)
            .with_background_color((50, 50, 50)),
        );
        strength_bar.add_event_handlers(&mut dom);
        agility_bar.add_event_handlers(&mut dom);
        defence_bar.add_event_handlers(&mut dom);

        dom.add_event_handler(1, |dom, _ctx, _node_id| {
            dom.select_mut(5).unwrap().set_visible(false);
        });

        dom.add_event_handler(20, move |_dom, ctx, _node_id| {
            let Rfid(Some(rfid)) = ctx.entity_component::<Rfid>(query_one!(ctx, Rfid)).clone()
            else {
                return;
            };
            let rfid = match rfid {
                HeroOrUnknownRfid::Hero(hero) => hero.rfid,
                HeroOrUnknownRfid::Rfid(_) => panic!("tried to update non existing hero"),
            };
            let menu = ctx
                .entity_component::<HeroCreator>(query_one!(ctx, HeroCreator))
                .clone();

            let stats = shared::HeroStats {
                strength: menu.strength_bar.lock().unwrap().steps_filled() as u8,
                agility: menu.agility_bar.lock().unwrap().steps_filled() as u8,
                defence: menu.defence_bar.lock().unwrap().steps_filled() as u8,
            };

            let comms = ctx.entity_component::<Comms>(query_one!(ctx, Comms));

            comms
                .req_sender
                .send(crate::Message::UpdateHeroStats(
                    shared::UpdateHeroStatsParams { rfid, stats },
                ))
                .unwrap()
        });

        use shared::HeroKind::*;
        for (id, hero_type) in [(10, Centrist), (11, Speed), (12, Strong), (13, Tankie)] {
            dom.add_event_handler(id, move |dom, ctx, _node_id| {
                let hero_type = hero_type.clone();
                let Rfid(Some(rfid)) = ctx.entity_component::<Rfid>(query_one!(ctx, Rfid)).clone()
                else {
                    return;
                };
                let rfid = match rfid {
                    HeroOrUnknownRfid::Hero(_) => panic!("tried to create existing hero"),
                    HeroOrUnknownRfid::Rfid(rfid) => rfid,
                };
                let comms = ctx.entity_component::<Comms>(query_one!(ctx, Comms));
                match comms
                    .req_sender
                    .send(crate::Message::CreateHero(shared::CreateHeroParams {
                        rfid,
                        hero_type: hero_type.clone() as _,
                        base_stats: shared::HeroStats::from(hero_type.clone()),
                    })) {
                    Ok(_) => {
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

    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        let since_last =
            ctx.entity_component::<SinceLastRequest>(query_one!(ctx, SinceLastRequest));
        since_last.0 += delta;

        if since_last.0 > 1.0 {
            since_last.0 = 0.0;
            let comms = ctx.entity_component::<Comms>(query_one!(ctx, Comms));
            comms.req_sender.send(crate::Message::BoardStatus).unwrap();
        }

        let menu = ctx
            .entity_component::<HeroCreator>(query_one!(ctx, HeroCreator))
            .clone();
        let mut dom = menu.dom.lock().unwrap();
        dom.update(ctx);

        menu.strength_bar.lock().unwrap().update(&mut dom);
        menu.agility_bar.lock().unwrap().update(&mut dom);
        menu.defence_bar.lock().unwrap().update(&mut dom);

        let comms = ctx.entity_component::<Comms>(query_one!(ctx, Comms));
        'handle_hero: {
            let Ok(hero) = comms.board_receiver.try_recv() else {
                break 'handle_hero;
            };
            dom.select_mut(50).unwrap().set_visible(false);

            let hero = match hero {
                Ok(v) => v,
                Err(err) => {
                    dom.select_mut(5).unwrap().set_visible(true);
                    change_text_node_content(
                        dom.select_mut(6),
                        format!("an error occurred: {err}"),
                    );
                    break 'handle_hero;
                }
            };
            use HeroOrUnknownRfid as HOUR;
            match hero {
                HOUR::Hero(hero) => {
                    let old_hero_info = ctx.entity_component::<Rfid>(query_one!(ctx, Rfid));
                    if let Some(ref old_rfid) = old_hero_info.0 {
                        match old_rfid {
                            HOUR::Hero(old_hero) if hero.rfid == old_hero.rfid => {
                                break 'handle_hero
                            }
                            _ => {}
                        };
                    }

                    change_text_node_content(
                        dom.select_mut(3),
                        format!("Available points: {}", hero.level * 3),
                    );

                    menu.strength_bar
                        .lock()
                        .unwrap()
                        .set_steps_filled(hero.strength_points as i32);
                    menu.agility_bar
                        .lock()
                        .unwrap()
                        .set_steps_filled(hero.agility_points as i32);
                    menu.defence_bar
                        .lock()
                        .unwrap()
                        .set_steps_filled(hero.defence_points as i32);

                    change_image_node_content(
                        dom.select_mut(2),
                        HeroInfo::from(&hero.hero_type).texture_path,
                    );

                    let Rfid(rfid) = ctx.entity_component::<Rfid>(query_one!(ctx, Rfid));
                    *rfid = Some(HOUR::Hero(hero));
                }
                HOUR::Rfid(rfid) => {
                    let old_rfid = ctx.entity_component::<Rfid>(query_one!(ctx, Rfid));
                    if let Some(ref old_rfid) = old_rfid.0 {
                        let old_rfid = match old_rfid {
                            HOUR::Hero(hero) => &hero.rfid,
                            HOUR::Rfid(rfid) => rfid,
                        };
                        if old_rfid == &rfid {
                            break 'handle_hero;
                        }
                    }
                    old_rfid.0 = Some(HOUR::Rfid(rfid));

                    dom.select_mut(4).unwrap().set_visible(true);
                }
            }
        }

        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}
