use std::{borrow::BorrowMut, sync::MutexGuard};

use engine::{query, spawn, Component, System};

use crate::{
    game::GameSystem,
    hero_info::HeroInfo,
    main_menu::MainMenuSystem,
    server::{Board, HeroResult, Res, Server},
    shared_ptr::SharedPtr,
    ui::{
        self,
        components::ProgressBar,
        focus::Focus,
        utils::{change_image_node_content, change_text_node_content},
    },
};

#[derive(Component, Clone)]
pub struct StartGame {
    system_id: u64,
    dom: SharedPtr<ui::Dom>,
    left_bars: SharedPtr<BarBundle>,
    right_bars: SharedPtr<BarBundle>,
    board_responder: Option<SharedPtr<Box<dyn Res<Board>>>>,
    focus: SharedPtr<Focus>,
}

#[repr(u64)]
enum Node {
    LeftImage,
    RightImage,
    ErrorText,
    ErrorPopup,
    ErrorPopupButton,
    LeftBars,
    RightBars,
    LeftOffset,
    RightOffset,
    StartGameButton,
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

#[derive(Component, Default, Clone)]
struct MaybeHeroesOnBoard(Option<crate::game::HeroesOnBoard>);

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
                            .id(Node::StartGameButton)
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
                        .id(Node::ErrorPopupButton)
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
            if let Some(heroes_on_board) = ctx.clone_one::<MaybeHeroesOnBoard>().0 {
                spawn!(ctx, heroes_on_board);
                ctx.remove_system(system_id);
                ctx.add_system(GameSystem);
            }
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
                }),
                board_responder: None,
                focus: SharedPtr::new(Focus::new([Node::StartGameButton, Node::ErrorPopupButton]))
            }
        );

        spawn!(ctx, MaybeHeroesOnBoard::default());

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        let start_game = ctx.clone_one::<StartGame>();
        let mut dom = start_game.dom.lock();
        dom.update(ctx);
        start_game.focus.lock().update(dom.borrow_mut(), ctx);

        let responder = match start_game.board_responder {
            Some(responder) => responder,
            None => {
                let responder = SharedPtr::new(ctx.select_one::<Server>().board_status());
                let start_game = ctx.select_one::<StartGame>();
                start_game.board_responder = Some(responder.clone());
                responder
            }
        };
        let heroes = responder.lock().try_receive();

        match heroes {
            Some(heroes) => {
                dom.select_mut(Node::ErrorPopup).unwrap().set_visible(false);

                display_hero_result(
                    heroes.hero_1.as_ref(),
                    Node::LeftImage,
                    Node::LeftBars,
                    Node::LeftOffset,
                    &mut dom,
                    start_game.left_bars.lock(),
                );
                display_hero_result(
                    heroes.hero_2.as_ref(),
                    Node::RightImage,
                    Node::RightBars,
                    Node::RightOffset,
                    &mut dom,
                    start_game.right_bars.lock(),
                );

                if let (Some(HeroResult::Hero(hero_1)), Some(HeroResult::Hero(hero_2))) =
                    (heroes.hero_1, heroes.hero_2)
                {
                    let heroes_on_board = ctx.select_one::<MaybeHeroesOnBoard>();
                    heroes_on_board
                        .0
                        .replace(crate::game::HeroesOnBoard { hero_1, hero_2 });
                }
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

fn display_hero_result(
    hero: Option<&HeroResult>,
    image_id: Node,
    bar_id: Node,
    offset_id: Node,
    dom: &mut MutexGuard<ui::Dom>,
    mut bars: MutexGuard<BarBundle>,
) {
    match hero {
        Some(HeroResult::Hero(hero)) => {
            change_image_node_content(
                dom.select_mut(image_id),
                HeroInfo::from(&hero.kind).texture_path,
            );
            bars.strength.set_steps_filled(hero.strength_points);
            bars.strength.update(dom);
            bars.agility.set_steps_filled(hero.agility_points);
            bars.agility.update(dom);
            bars.defence.set_steps_filled(hero.defence_points);
            bars.defence.update(dom);

            dom.select_mut(bar_id).unwrap().set_visible(true);
            dom.select_mut(offset_id).unwrap().set_visible(false);
        }
        erronous => {
            let error = match erronous {
                Some(HeroResult::UnknownRfid(_)) => {
                    "Atleast 1 hero is not initialized, please go to the hero creator."
                }
                None => "No hero found",
                _ => unreachable!(),
            };

            change_text_node_content(dom.select_mut(Node::ErrorText), error);
            dom.select_mut(Node::ErrorPopup).unwrap().set_visible(true);

            dom.select_mut(bar_id).unwrap().set_visible(false);
            dom.select_mut(offset_id).unwrap().set_visible(true);
        }
    };
}
