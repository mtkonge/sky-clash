use std::sync::MutexGuard;

use engine::{query, spawn, Component, System};
use shared::Hero;

use crate::{
    game::GameSystem,
    hero_info::HeroInfo,
    main_menu::MainMenuSystem,
    server::HeroResult,
    shared_ptr::SharedPtr,
    ui::{
        self,
        components::ProgressBar,
        utils::{change_image_node_content, change_text_node_content},
    },
    GameActor,
};

#[derive(Component, Clone)]
pub struct StartGame {
    system_id: u64,
    dom: SharedPtr<ui::Dom>,
    left_bars: SharedPtr<BarBundle>,
    right_bars: SharedPtr<BarBundle>,
}

fn handle_hero_result<I: Into<ui::NodeId>>(
    hero: Option<HeroResult>,
    dom_id: I,
    dom: &mut MutexGuard<ui::Dom>,
    mut bars: MutexGuard<BarBundle>,
) -> Result<Hero, String> {
    match hero {
        Some(hero) => match hero {
            HeroResult::Hero(hero) => {
                change_image_node_content(
                    dom.select_mut(dom_id),
                    HeroInfo::from(&hero.hero_type).texture_path,
                );
                bars.strength.set_steps_filled(hero.strength_points);
                bars.agility.set_steps_filled(hero.agility_points);
                bars.defence.set_steps_filled(hero.defence_points);
                Ok(hero)
            }
            HeroResult::UnknownRfid(_) => {
                change_text_node_content(
                    dom.select_mut(Node::ErrorText),
                    "Atleast 1 hero is not initialized, please go to the hero creator.",
                );
                dom.select_mut(Node::ErrorPopup).unwrap().set_visible(true);
                Err("uhhmm hero with rfid does not acshually exist :nerd:".to_string())
            }
        },
        None => Err("No hero found".to_string()),
    }
}

#[repr(u64)]
enum Node {
    LeftImage,
    RightImage,
    ErrorText,
    ErrorPopup,
    LeftBars,
    RightBars,
    LeftOffset,
    RightOffset,
}

#[repr(u64)]
enum Event {
    StartGame,
    ErrorPopupClick,
}

impl From<Node> for ui::NodeId {
    fn from(value: Node) -> Self {
        Self::from_u64(value as u64)
    }
}

impl From<Event> for ui::EventId {
    fn from(value: Event) -> Self {
        Self::from_u64(value as u64)
    }
}

struct BarBundle {
    strength: ProgressBar,
    agility: ProgressBar,
    defence: ProgressBar,
}

pub struct StartGameSystem(pub u64);
impl System for StartGameSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui::components::*;
        use ui::constructors::*;

        let system_id = self.0;

        let left_strength_bar = ProgressBar::new_immutable("Strength", 24);
        let left_agility_bar = ProgressBar::new_immutable("Agility", 24);
        let left_defence_bar = ProgressBar::new_immutable("Defence", 24);

        let right_strength_bar = ProgressBar::new_immutable("Strength", 24);
        let right_agility_bar = ProgressBar::new_immutable("Agility", 24);
        let right_defence_bar = ProgressBar::new_immutable("Defence", 24);

        let mut dom = ui::Dom::new(
            Stack([
                Hori([
                    Vert([
                        Rect().height(100),
                        Rect().height(150).width(200).id(Node::LeftOffset),
                        Vert([
                            left_strength_bar.build(),
                            left_agility_bar.build(),
                            left_defence_bar.build(),
                        ])
                        .height(150)
                        .width(200)
                        .visible(false)
                        .id(Node::LeftBars),
                        Image("./textures/placeholder.png")
                            .id(Node::LeftImage)
                            .width(200)
                            .height(200)
                            .background_color((255, 0, 0)),
                    ]),
                    Rect().width(200),
                    Vert([
                        Rect().height(400),
                        Button("Start Game")
                            .color((255, 255, 255))
                            .padding(15)
                            .on_click(Event::StartGame),
                    ]),
                    Rect().width(200),
                    Vert([
                        Rect().height(100),
                        Rect().height(150).width(200).id(Node::RightOffset),
                        Vert([
                            right_strength_bar.build(),
                            right_agility_bar.build(),
                            right_defence_bar.build(),
                        ])
                        .height(150)
                        .width(200)
                        .visible(false)
                        .id(Node::RightBars),
                        Image("./textures/placeholder.png")
                            .id(Node::RightImage)
                            .width(200)
                            .height(200)
                            .background_color((255, 0, 0)),
                    ]),
                ]),
                Vert([
                    Text("Error").id(Node::ErrorText).padding(5),
                    Button("Ok")
                        .background_color((100, 100, 100))
                        .padding(5)
                        .on_click(Event::ErrorPopupClick),
                ])
                .padding(5)
                .border_thickness(2)
                .visible(false)
                .id(Node::ErrorPopup),
            ])
            .background_color((50, 50, 50))
            .width(1280)
            .height(720),
        );

        dom.add_event_handler(Event::StartGame, move |_dom, ctx, _node_id| {
            ctx.remove_system(system_id);
            ctx.add_system(GameSystem);
        });

        dom.add_event_handler(Event::ErrorPopupClick, move |dom, ctx, _node_id| {
            ctx.remove_system(system_id);
            ctx.add_system(MainMenuSystem);
            dom.select_mut(Node::ErrorPopup).unwrap().set_visible(false)
        });

        spawn!(
            ctx,
            StartGame {
                system_id: self.0,
                dom: SharedPtr::new(dom),
                left_bars: SharedPtr::new(BarBundle {
                    strength: left_strength_bar,
                    agility: left_agility_bar,
                    defence: left_defence_bar
                }),
                right_bars: SharedPtr::new(BarBundle {
                    strength: right_strength_bar,
                    agility: right_agility_bar,
                    defence: right_defence_bar
                })
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        let start_game = ctx.clone_one::<StartGame>();
        start_game.dom.lock().update(ctx);

        let mut dom = start_game.dom.lock();

        let comms = ctx.select_one::<GameActor>();
        comms.server.send(crate::Message::BoardStatus);

        let heroes = comms.inner.try_receive();

        match heroes {
            Some(heroes) => {
                match handle_hero_result(
                    heroes.hero_1,
                    Node::LeftImage,
                    &mut dom,
                    start_game.left_bars.lock(),
                ) {
                    Ok(_) => {
                        dom.select_mut(Node::LeftBars).unwrap().set_visible(true);
                        dom.select_mut(Node::LeftOffset).unwrap().set_visible(false);
                    }
                    Err(err) => {
                        dom.select_mut(Node::LeftBars).unwrap().set_visible(false);
                        dom.select_mut(Node::LeftOffset).unwrap().set_visible(true);

                        println!("{}", err);
                    }
                };
                match handle_hero_result(
                    heroes.hero_2,
                    Node::RightImage,
                    &mut dom,
                    start_game.right_bars.lock(),
                ) {
                    Ok(_) => {
                        dom.select_mut(Node::RightBars).unwrap().set_visible(true);
                        dom.select_mut(Node::RightOffset)
                            .unwrap()
                            .set_visible(false);
                    }
                    Err(err) => {
                        dom.select_mut(Node::RightBars).unwrap().set_visible(false);
                        dom.select_mut(Node::RightOffset).unwrap().set_visible(true);
                        println!("{}", err);
                    }
                };
            }
            None => return Ok(()),
        }
        Ok(())
    }

    fn on_remove(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        for id in query!(ctx, StartGame) {
            let start_game = ctx.select::<StartGame>(id).clone();
            if start_game.system_id == self.0 {
                ctx.despawn(id);
            }
        }

        Ok(())
    }
}
