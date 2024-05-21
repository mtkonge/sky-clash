use crate::hero_info::HeroInfo;
use crate::main_menu::MainMenuSystem;
use crate::server::Board;
use crate::server::HeroResult;
use crate::server::Res;
use crate::server::Server;
use crate::shared_ptr::SharedPtr;
use crate::ui;
use crate::ui::utils::change_image_node_content;
use crate::ui::utils::change_text_node_content;
use engine::query_one;
use engine::spawn;
use engine::{Component, System};

#[derive(Component, Clone)]
pub struct HeroCreator {
    dom: SharedPtr<ui::Dom>,
    focus: SharedPtr<ui::focus::Focus>,
    unallocated_skill_points: SharedPtr<i64>,
    strength_bar: SharedPtr<ui::components::ProgressBar>,
    defence_bar: SharedPtr<ui::components::ProgressBar>,
    agility_bar: SharedPtr<ui::components::ProgressBar>,
    hero: Option<HeroResult>,
    board_responder: Option<SharedPtr<Box<dyn Res<Board>>>>,
}

#[repr(u64)]
enum Node {
    HeroTypeText,
    HeroImage,
    AvailablePoints,
    HeroKindPopup,
    ErrorPopup,
    ErrorText,
    Loading,

    ClosePopup,
    UpdateHero,
    CentristButton,
    StrongButton,
    TankieButton,
    SpeedButton,
}

#[repr(u64)]
enum Event {
    ClosePopup,
    UpdateHero,
    CentristButton,
    StrongButton,
    TankieButton,
    SpeedButton,
}

impl From<Node> for ui::NodeId {
    fn from(value: Node) -> Self {
        ui::NodeId::from_u64(value as u64)
    }
}

impl From<Event> for ui::EventId {
    fn from(value: Event) -> Self {
        ui::EventId::from_u64(value as u64)
    }
}

pub struct HeroCreatorSystem(pub u64);
impl System for HeroCreatorSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui::components::*;
        let strength_bar = ProgressBar::new("Strength", 24);
        let agility_bar = ProgressBar::new("Agility", 24);
        let defence_bar = ProgressBar::new("Defence", 24);

        let mut dom = self.build_dom(&strength_bar, &agility_bar, &defence_bar);

        strength_bar.add_event_handlers(&mut dom);
        agility_bar.add_event_handlers(&mut dom);
        defence_bar.add_event_handlers(&mut dom);

        let id = self.0;
        dom.add_event_handler(Event::ClosePopup, move |_dom, ctx, _node_id| {
            ctx.remove_system(id);
            ctx.add_system(MainMenuSystem);
        });

        dom.add_event_handler(Event::UpdateHero, move |_dom, ctx, _node_id| {
            let menu = ctx.clone_one::<HeroCreator>();
            let Some(hero) = menu.hero else {
                return;
            };
            let rfid = match hero {
                HeroResult::Hero(hero) => hero.rfid,
                HeroResult::UnknownRfid(_) => panic!("tried to update non existing hero"),
            };

            let stats = shared::HeroStats {
                strength: menu.strength_bar.lock().steps_filled(),
                agility: menu.agility_bar.lock().steps_filled(),
                defence: menu.defence_bar.lock().steps_filled(),
            };

            let server = ctx.select_one::<Server>();
            server.update_hero_stats(shared::UpdateHeroStatsParams { rfid, stats });
        });

        use shared::HeroKind::*;
        for (id, hero_type) in [
            (Event::CentristButton, Centrist),
            (Event::SpeedButton, Speed),
            (Event::StrongButton, Strong),
            (Event::TankieButton, Tankie),
        ] {
            dom.add_event_handler(id, move |_dom, ctx, _node_id| {
                let hero_type = hero_type.clone();
                let menu = ctx.clone_one::<HeroCreator>();
                let Some(hero) = menu.hero else {
                    return;
                };
                let rfid = match hero {
                    HeroResult::Hero(_) => panic!("tried to create existing hero"),
                    HeroResult::UnknownRfid(rfid) => rfid,
                };

                let server = ctx.select_one::<Server>();
                server.create_hero(shared::CreateHeroParams {
                    rfid,
                    hero_type: hero_type.clone(),
                    base_stats: shared::HeroStats::from(hero_type.clone()),
                });
            });
        }

        spawn!(
            ctx,
            HeroCreator {
                dom: SharedPtr::new(dom),
                strength_bar: SharedPtr::new(strength_bar),
                agility_bar: SharedPtr::new(agility_bar),
                defence_bar: SharedPtr::new(defence_bar),
                unallocated_skill_points: SharedPtr::new(0),
                hero: None,
                board_responder: None,
                focus: SharedPtr::new(ui::focus::Focus::new([
                    Node::ClosePopup,
                    Node::UpdateHero,
                    Node::CentristButton,
                    Node::StrongButton,
                    Node::TankieButton,
                    Node::SpeedButton,
                ]))
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        let menu = ctx.clone_one::<HeroCreator>();
        let mut dom = menu.dom.lock();
        let mut focus = menu.focus.lock();
        focus.update(&mut dom, ctx);
        dom.update(ctx);

        if let Some(HeroResult::Hero(hero)) = menu.hero {
            let total_allocated = [&menu.strength_bar, &menu.agility_bar, &menu.defence_bar]
                .into_iter()
                .map(|bar| bar.lock().steps_filled())
                .sum::<i64>();

            change_text_node_content(
                dom.select_mut(Node::AvailablePoints),
                format!(
                    "Available points: {}",
                    hero.total_skill_points() - total_allocated
                ),
            );

            let unallocated = hero.total_skill_points() - total_allocated;
            for bar in [&menu.strength_bar, &menu.agility_bar, &menu.defence_bar] {
                let filled = bar.lock().steps_filled();
                bar.lock().set_upper_limit(filled + unallocated);
            }
        }

        menu.strength_bar.lock().update(&mut dom);
        menu.agility_bar.lock().update(&mut dom);
        menu.defence_bar.lock().update(&mut dom);
        self.try_receive_and_update_hero(ctx, dom);

        Ok(())
    }

    fn on_remove(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        let id = query_one!(ctx, HeroCreator);
        ctx.despawn(id);
        Ok(())
    }
}

impl HeroCreatorSystem {
    fn build_dom(
        &self,
        strength_bar: &ui::components::ProgressBar,
        agility_bar: &ui::components::ProgressBar,
        defence_bar: &ui::components::ProgressBar,
    ) -> ui::Dom {
        use ui::components::*;
        use ui::constructors::*;
        ui::Dom::new(
            Stack([
                Hori([
                    Vert([
                        Vert([
                            Image("./textures/player.png")
                                .id(Node::HeroImage)
                                .width(128)
                                .height(128),
                            Text("Boykisser").id(Node::HeroTypeText).padding(30),
                            Rect().height(720 / 16),
                        ])
                        .padding(50)
                        .border_thickness(2),
                        Rect().height(720 / 4),
                    ]),
                    Rect().width(1280 / 4),
                    Vert([
                        Text("Available points: 0").id(Node::AvailablePoints),
                        strength_bar.build(),
                        agility_bar.build(),
                        defence_bar.build(),
                        Hori([ui::components::Button("Confirm")
                            .on_click(Event::UpdateHero)
                            .id(Node::UpdateHero)]),
                        Rect().height(720 / 2 - 100),
                    ]),
                ]),
                Vert([
                    Text("Unknown hero").padding(5),
                    Text("Please select which hero this is").padding(5),
                    Hori([
                        Button("Centrist")
                            .background_color((100, 100, 100))
                            .padding(5)
                            .on_click(Event::CentristButton)
                            .id(Node::CentristButton),
                        Button("Speed")
                            .background_color((100, 100, 100))
                            .padding(5)
                            .on_click(Event::SpeedButton)
                            .id(Node::SpeedButton),
                        Button("Strong")
                            .background_color((100, 100, 100))
                            .padding(5)
                            .on_click(Event::StrongButton)
                            .id(Node::StrongButton),
                        Button("Tankie")
                            .background_color((100, 100, 100))
                            .padding(5)
                            .on_click(Event::TankieButton)
                            .id(Node::TankieButton),
                    ]),
                ])
                .id(Node::HeroKindPopup)
                .visible(false)
                .border_thickness(2)
                .padding(5),
                Vert([
                    Text("Error").id(Node::ErrorText).padding(5),
                    Button("Ok")
                        .background_color((100, 100, 100))
                        .padding(5)
                        .on_click(Event::ClosePopup)
                        .id(Node::ClosePopup),
                ])
                .id(Node::ErrorPopup)
                .visible(false)
                .border_thickness(2)
                .padding(5),
                Text("Loading...")
                    .width(1280)
                    .height(720)
                    .background_color((50, 50, 50))
                    .id(Node::Loading),
            ])
            .width(1280)
            .height(720)
            .background_color((50, 50, 50)),
        )
    }

    fn try_receive_and_update_hero(
        &self,
        ctx: &mut engine::Context,
        mut dom: std::sync::MutexGuard<ui::Dom>,
    ) {
        let menu = ctx.select_one::<HeroCreator>();
        let responder = match menu.board_responder.clone() {
            Some(responder) => responder,
            None => {
                let server = ctx.select_one::<Server>();
                let responder = SharedPtr::new(server.board_status());
                let menu = ctx.select_one::<HeroCreator>();
                menu.board_responder = Some(responder.clone());
                responder
            }
        };

        let Some(hero) = responder.lock().try_receive() else {
            return;
        };

        let menu = ctx.select_one::<HeroCreator>();
        menu.board_responder = None;

        dom.select_mut(Node::Loading).unwrap().set_visible(false);

        let hero = match hero {
            Board {
                hero_1: Some(hero),
                hero_2: None,
            }
            | Board {
                hero_1: None,
                hero_2: Some(hero),
            } => Ok(hero),
            Board {
                hero_1: None,
                hero_2: None,
            } => Err("please put 1 hero on board"),
            Board {
                hero_1: Some(_),
                hero_2: Some(_),
            } => Err("please put only 1 hero on board"),
        };

        let hero = match hero {
            Ok(hero) => hero,
            Err(err) => {
                change_text_node_content(dom.select_mut(Node::ErrorText), err);
                dom.select_mut(Node::ErrorPopup).unwrap().set_visible(true);
                return;
            }
        };

        match hero {
            HeroResult::Hero(hero) => initialize_hero(ctx, hero, dom),
            HeroResult::UnknownRfid(rfid) => {
                let menu = ctx.select_one::<HeroCreator>();
                let old_hero = &menu.hero;
                if let Some(ref old_hero) = old_hero {
                    let old_rfid = match old_hero {
                        HeroResult::Hero(hero) => &hero.rfid,
                        HeroResult::UnknownRfid(rfid) => rfid,
                    };
                    if old_rfid == &rfid {
                        return;
                    }
                }
                menu.hero = Some(HeroResult::UnknownRfid(rfid));
                dom.select_mut(Node::HeroKindPopup)
                    .unwrap()
                    .set_visible(true);
            }
        }
    }
}

fn initialize_hero(
    ctx: &mut engine::Context,
    hero: shared::Hero,
    mut dom: std::sync::MutexGuard<ui::Dom>,
) {
    dom.select_mut(Node::HeroKindPopup)
        .unwrap()
        .set_visible(false);

    let menu = ctx.select_one::<HeroCreator>();
    let old_hero_info = &menu.hero;
    if let Some(old_hero) = old_hero_info {
        match old_hero {
            HeroResult::Hero(old_hero) if hero.rfid == old_hero.rfid => {
                return;
            }
            _ => {}
        };
    }

    change_text_node_content(
        dom.select_mut(Node::AvailablePoints),
        format!("Available points: {}", hero.unallocated_skill_points()),
    );
    let hero_info = HeroInfo::from(&hero.kind);
    menu.strength_bar
        .lock()
        .set_steps_filled(hero.strength_points)
        .set_lower_limit(hero.strength_points);
    menu.agility_bar
        .lock()
        .set_steps_filled(hero.agility_points)
        .set_lower_limit(hero.agility_points);
    menu.defence_bar
        .lock()
        .set_steps_filled(hero.defence_points)
        .set_lower_limit(hero.defence_points);
    change_text_node_content(dom.select_mut(Node::HeroTypeText), hero_info.name);
    change_image_node_content(dom.select_mut(Node::HeroImage), hero_info.texture_path);
    menu.hero = Some(HeroResult::Hero(hero));
}
