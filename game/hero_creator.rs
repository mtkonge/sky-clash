use super::{
    engine::{self, System},
    ui,
};
use crate::spawn;
use std::rc::Rc;

pub struct MainMenu;
impl System for MainMenu {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        ctx.add_system(ui::TitleSystem);
        ctx.add_system(ui::ButtonSystem);
        let texture = ctx
            .render_text("textures/ttf/OpenSans.ttf", "SkyCwash", 48, (255, 255, 255))
            .unwrap();
        spawn!(
            ctx,
            ui::Title {
                pos: (400, 100),
                texture
            }
        );
        let texture = ctx
            .render_text(
                "textures/ttf/OpenSans.ttf",
                "Pway gwame",
                24,
                (255, 255, 255),
            )
            .unwrap();
        spawn!(
            ctx,
            ui::Button {
                pos: (400, 250),
                size: (400, 80),
                texture,
                action: Rc::new(|_| loop {})
            },
        );
        let texture = ctx
            .render_text(
                "textures/ttf/OpenSans.ttf",
                "Hewo cweator",
                24,
                (255, 255, 255),
            )
            .unwrap();
        spawn!(
            ctx,
            ui::Button {
                pos: (400, 350),
                size: (400, 80),
                texture,
                action: Rc::new(|_| ())
            },
        );
        let texture = ctx
            .render_text(
                "textures/ttf/OpenSans.ttf",
                "Exit gwame T~T",
                24,
                (255, 255, 255),
            )
            .unwrap();
        spawn!(
            ctx,
            ui::Button {
                pos: (400, 450),
                size: (400, 80),
                texture,
                action: Rc::new(|_| panic!())
            },
        );
        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        ctx.draw_rect((0, 0, 0), 0, 0, 1280, 720)?;
        Ok(())
    }
}
