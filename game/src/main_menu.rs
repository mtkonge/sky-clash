use crate::hero_creator::HeroCreatorSystem;
use crate::shared_ptr::SharedPtr;
use crate::start_game::StartGameSystem;
use crate::ui;
use crate::ui::components::Button;
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
        use ui::constructors::*;

        let system_id = self.0;

        let mut dom = ui::Dom::new(
            Stack([Vert([
                Text("SkyTrash").font_size(48),
                Button("Start Game")
                    .color((255, 255, 255))
                    .padding(15)
                    .border_thickness(2)
                    .id(Node::StartGame)
                    .on_click(Event::StartGame),
                Button("Hero Creator")
                    .color((255, 255, 255))
                    .padding(15)
                    .border_thickness(2)
                    .id(Node::HeroCreator)
                    .on_click(Event::HeroCreator),
                Button("Exit")
                    .color((255, 255, 255))
                    .padding(15)
                    .border_thickness(2)
                    .id(Node::Exit)
                    .on_click(Event::Exit),
            ])])
            .background_color((50, 50, 50))
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
