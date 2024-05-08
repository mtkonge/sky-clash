use crate::hero_info::HeroInfo;
use crate::server::HeroResult;
use crate::shared_ptr::SharedPtr;
use crate::ui;
use crate::GameActor;
use engine::spawn;
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

pub fn change_image_node_content<P: Into<std::path::PathBuf>>(
    node: Option<&mut ui::Node>,
    new_path: P,
) {
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
    dom: SharedPtr<ui::Dom>,
    unallocated_skill_points: SharedPtr<i64>,
    strength_bar: SharedPtr<ui::components::ProgressBar>,
    defence_bar: SharedPtr<ui::components::ProgressBar>,
    agility_bar: SharedPtr<ui::components::ProgressBar>,
    hero: Option<HeroResult>,
}

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
pub struct HeroCreatorSystem(pub u64);
impl System for HeroCreatorSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        let strength_bar = ui::components::ProgressBar::new("Strength", 24, 100);
        let agility_bar = ui::components::ProgressBar::new("Agility", 24, 300);
        let defence_bar = ui::components::ProgressBar::new("Defence", 24, 200);

        let mut dom = self.build_dom(&strength_bar, &agility_bar, &defence_bar);

        strength_bar.add_event_handlers(&mut dom);
        agility_bar.add_event_handlers(&mut dom);
        defence_bar.add_event_handlers(&mut dom);

        dom.add_event_handler(1, |dom, _ctx, _node_id| {
            dom.select_mut(5).unwrap().set_visible(false);
        });

        dom.add_event_handler(20, move |_dom, ctx, _node_id| {
            let menu = ctx.clone_one::<HeroCreator>();
            let Some(hero) = menu.hero else {
                return;
            };
            let rfid = match hero {
                HeroResult::Hero(hero) => hero.rfid,
                HeroResult::UnknownRfid(_) => panic!("tried to update non existing hero"),
            };

            let stats = shared::HeroStats {
                strength: menu.strength_bar.lock().steps_filled() as u8,
                agility: menu.agility_bar.lock().steps_filled() as u8,
                defence: menu.defence_bar.lock().steps_filled() as u8,
            };

            let comms = ctx.select_one::<GameActor>();

            comms.server.send_important(crate::Message::UpdateHeroStats(
                shared::UpdateHeroStatsParams { rfid, stats },
            ))
        });

        use shared::HeroKind::*;
        for (id, hero_type) in [(10, Centrist), (11, Speed), (12, Strong), (13, Tankie)] {
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
                let comms = ctx.select_one::<GameActor>();
                comms
                    .server
                    .send_important(crate::Message::CreateHero(shared::CreateHeroParams {
                        rfid,
                        hero_type: hero_type.clone() as _,
                        base_stats: shared::HeroStats::from(hero_type.clone()),
                    }));
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
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        let menu = ctx.clone_one::<HeroCreator>();
        let mut dom = menu.dom.lock();
        dom.update(ctx);

        if let Some(HeroResult::Hero(hero)) = menu.hero {
            let total_allocated = [&menu.strength_bar, &menu.agility_bar, &menu.defence_bar]
                .into_iter()
                .map(|bar| bar.lock().steps_filled())
                .sum::<i64>();

            change_text_node_content(
                dom.select_mut(3),
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

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
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
                                .with_id(NodeId::HeroImage)
                                .with_width(128)
                                .with_height(128),
                            Text("Boykisser")
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
        )
    }

    fn try_receive_and_update_hero(
        &self,
        ctx: &mut engine::Context,
        mut dom: std::sync::MutexGuard<ui::Dom>,
    ) {
        let comms = ctx.select_one::<GameActor>();
        comms.server.send(crate::Message::BoardStatus);
        let Some(hero) = comms.inner.try_receive() else {
            return;
        };
        dom.select_mut(50).unwrap().set_visible(false);

        let hero = match hero {
            Ok(v) => v,
            Err(err) => {
                dom.select_mut(5).unwrap().set_visible(true);
                change_text_node_content(dom.select_mut(6), format!("an error occurred: {err}"));
                return;
            }
        };
        match hero {
            HeroResult::Hero(hero) => update_hero(ctx, hero, dom),
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

                dom.select_mut(4).unwrap().set_visible(true);
            }
        }
    }
}

fn update_hero(
    ctx: &mut engine::Context,
    hero: shared::Hero,
    mut dom: std::sync::MutexGuard<ui::Dom>,
) {
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
        dom.select_mut(3),
        format!("Available points: {}", hero.unallocated_skill_points()),
    );
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
    let hero_info = HeroInfo::from(&hero.hero_type);
    change_text_node_content(dom.select_mut(NodeId::HeroTypeText as u64), hero_info.name);
    change_image_node_content(
        dom.select_mut(NodeId::HeroImage as u64),
        hero_info.texture_path,
    );
    menu.hero = Some(HeroResult::Hero(hero));
}
