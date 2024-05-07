use engine::{query, spawn, Component, System};

use crate::{game::GameSystem, shared_ptr::SharedPtr, ui};

#[derive(Component, Clone)]
pub struct StartGame {
    system_id: u64,
    dom: SharedPtr<ui::Dom>,
}

pub struct StartGameSystem(pub u64);
impl System for StartGameSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui::components::*;
        use ui::constructors::*;

        let system_id = self.0;

        let mut dom = ui::Dom::new(
            Stack([Hori([
                Vert([
                    Rect().with_height(300),
                    Rect()
                        .with_width(200)
                        .with_height(200)
                        .with_background_color((255, 0, 0)),
                ]),
                Rect().with_width(200),
                Vert([
                    Rect().with_height(400),
                    Button("Start Game")
                        .with_color((255, 255, 255))
                        .with_padding(15)
                        .on_click(0),
                ]),
                Rect().with_width(200),
                Vert([
                    Rect().with_height(300),
                    Rect()
                        .with_width(200)
                        .with_height(200)
                        .with_background_color((0, 255, 0)),
                ]),
            ])])
            .with_background_color((50, 50, 50))
            .with_width(1280)
            .with_height(720),
        );

        dom.add_event_handler(0, move |_dom, ctx, _node_id| {
            ctx.remove_system(system_id);
            ctx.add_system(GameSystem);
        });
        spawn!(
            ctx,
            StartGame {
                system_id: self.0,
                dom: SharedPtr::new(dom)
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, StartGame) {
            let start_game = ctx.select::<StartGame>(id).clone();
            start_game.dom.lock().update(ctx);
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
