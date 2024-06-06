use crate::hero_creator::HeroCreatorSystem;
use crate::start_game::StartGameSystem;
use engine::ui;
use engine::ui::components::Button;
use engine::SharedPtr;
use engine::{query, spawn};
use engine::{Component, System};

#[derive(Component, Clone)]
pub struct MainMenu {
    system_id: u64,
    dom: SharedPtr<ui::Dom>,
    focus: SharedPtr<ui::focus::Focus>,
}

#[repr(u64)]
#[derive(Clone)]
pub enum Node {
    StartGame,
    HeroCreator,
    Exit,
}

impl From<Node> for ui::NodeId {
    fn from(value: Node) -> Self {
        Self::from_u64(value as u64)
    }
}

#[repr(u64)]
pub enum Event {
    StartGame,
    HeroCreator,
    Exit,
}

impl From<Event> for ui::EventId {
    fn from(value: Event) -> Self {
        Self::from_u64(value as u64)
    }
}

pub struct MainMenuSystem(pub u64);
impl System for MainMenuSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui::constructors::{Hori, Image, Rect, Stack, Text, Vert};

        let system_id = self.0;

        let mut dom = ui::Dom::new(
            Stack([
                Image("assets/main_menu.png").width(1280).height(720),
                Vert([
                    Stack([
                        Vert([
                            Rect().height(20),
                            Hori([
                                Rect().width(20),
                                Text("Sky Clash").color((0, 0, 0)).font_size(100),
                            ]),
                        ]),
                        Text("Sky Clash").font_size(100),
                    ]),
                    Rect().height(100),
                    Button("Start Game")
                        .width(200)
                        .color((255, 255, 255))
                        .background_color((50, 50, 50))
                        .padding(15)
                        .border_thickness(2)
                        .id(Node::StartGame)
                        .on_click(Event::StartGame),
                    Button("Hero Creator")
                        .width(200)
                        .color((255, 255, 255))
                        .background_color((50, 50, 50))
                        .padding(15)
                        .border_thickness(2)
                        .id(Node::HeroCreator)
                        .on_click(Event::HeroCreator),
                    Button("Exit")
                        .width(200)
                        .color((255, 255, 255))
                        .background_color((50, 50, 50))
                        .padding(15)
                        .border_thickness(2)
                        .id(Node::Exit)
                        .on_click(Event::Exit),
                ])
                .gap(8),
            ])
            .background_color((50, 50, 50))
            .font_size(20)
            .width(1280)
            .height(720),
        );

        dom.add_event_handler(Event::StartGame, move |_dom, ctx, _node_id| {
            ctx.remove_system(system_id);
            ctx.add_system(StartGameSystem);
        });

        dom.add_event_handler(Event::HeroCreator, move |_dom, ctx, _node_id| {
            ctx.remove_system(system_id);
            ctx.add_system(HeroCreatorSystem);
        });

        dom.add_event_handler(Event::Exit, |_dom, _ctx, _node_id| {
            panic!("exit");
        });

        spawn!(
            ctx,
            MainMenu {
                system_id: self.0,
                dom: SharedPtr::new(dom),
                focus: SharedPtr::new(ui::focus::Focus::new([
                    Node::StartGame,
                    Node::HeroCreator,
                    Node::Exit
                ])),
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, MainMenu) {
            let main_menu = ctx.select::<MainMenu>(id).clone();
            let mut dom = main_menu.dom.lock();
            let mut focus = main_menu.focus.lock();
            focus.update(&mut dom, ctx);
            dom.update(ctx);
        }
        Ok(())
    }

    fn on_remove(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        for id in query!(ctx, MainMenu) {
            let main_menu = ctx.select::<MainMenu>(id).clone();
            if main_menu.system_id == self.0 {
                ctx.despawn(id);
            }
        }
        Ok(())
    }
}
