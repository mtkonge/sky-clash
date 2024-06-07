use crate::game::Game;
use crate::main_menu::MainMenuSystem;
use crate::ui_components::Button;
use engine::ui;
use engine::ui::focus::Focus;
use engine::Component;
use engine::SharedPtr;
use engine::System;
use engine::{query, spawn};

#[derive(Component, Clone)]
pub struct Pause {
    dom: SharedPtr<ui::Dom>,
    system_id: u64,
    focus: SharedPtr<Focus>,
}

#[repr(u64)]
enum Node {
    ResumeButton,
    MainMenuButton,
}

impl From<Node> for ui::NodeId {
    fn from(value: Node) -> Self {
        Self::from_u64(value as u64)
    }
}

enum Event {
    Resume,
    ReturnToMenu,
}

impl From<Event> for ui::EventId {
    fn from(value: Event) -> Self {
        Self::from_u64(value as u64)
    }
}

pub struct PauseSystem(pub u64);

impl System for PauseSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui::constructors::{Rect, Text, Vert};

        let system_id = self.0;

        let mut dom = ui::Dom::new(
            Vert([
                //
                Rect().height(100),
                Text("Paused").font_size(70),
                Rect().height(20),
                Button("Resume")
                    .width(200)
                    .color((255, 255, 255))
                    .background_color((50, 50, 50))
                    .padding(15)
                    .border_thickness(2)
                    .id(Node::ResumeButton)
                    .on_click(Event::Resume),
                Button("Main menu")
                    .width(200)
                    .color((255, 255, 255))
                    .background_color((50, 50, 50))
                    .padding(15)
                    .border_thickness(2)
                    .id(Node::MainMenuButton)
                    .on_click(Event::ReturnToMenu),
            ])
            .gap(8)
            .width(1280)
            .height(720),
        );

        dom.add_event_handler(Event::Resume, move |_dom, ctx, _node_id| {
            ctx.remove_system(system_id);

            let game = ctx.select_one::<Game>();
            game.paused = false;
        });

        dom.add_event_handler(Event::ReturnToMenu, move |_dom, ctx, _node_id| {
            ctx.remove_system(system_id);

            let game = ctx.select_one::<Game>().system_id;
            ctx.remove_system(game);
            ctx.add_system(MainMenuSystem);
        });

        spawn!(
            ctx,
            Pause {
                dom: dom.into(),
                system_id: self.0,
                focus: SharedPtr::new(Focus::new([Node::ResumeButton, Node::MainMenuButton,])),
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        let _ = ctx.draw_rect_alpha((0, 0, 0), 100, 0, 0, 1280, 720);

        for id in query!(ctx, Pause) {
            let pause = ctx.select::<Pause>(id).clone();
            let mut dom = pause.dom.lock();
            let mut focus = pause.focus.lock();
            focus.update(&mut dom, ctx);
            dom.update(ctx);
        }

        Ok(())
    }

    fn on_remove(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        for id in query!(ctx, Pause) {
            let pause = ctx.select::<Pause>(id).clone();
            if pause.system_id == self.0 {
                ctx.despawn(id);
            }
        }

        Ok(())
    }
}
