use super::{
    engine::{self, System},
    ui,
};
use crate::spawn;
use std::rc::Rc;

pub struct MainMenu;

impl System for MainMenu {
    fn on_remove(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        ctx.remove_system::<ui::TitleSystem>();
        ctx.remove_system::<ui::ButtonSystem>();
        todo!("remove currently loaded buttons/titles");
        Ok(())
    }

    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        let font48 = ctx.load_font("textures/ttf/OpenSans.ttf", 48)?;
        let font24 = ctx.load_font("textures/ttf/OpenSans.ttf", 24)?;
        ctx.add_system(ui::TitleSystem);
        ctx.add_system(ui::ButtonSystem);
        let texture = ctx
            .render_text(font48, "SkyCwash", (255, 255, 255))
            .unwrap();
        spawn!(
            ctx,
            ui::Title {
                pos: (400, 100),
                texture
            }
        );
        let texture = ctx
            .render_text(font24, "Pway gwame", (255, 255, 255))
            .unwrap();
        spawn!(
            ctx,
            ui::Button {
                pos: (400, 250),
                size: (400, 80),
                texture,
                action: Rc::new(|_| ())
            },
        );
        let texture = ctx
            .render_text(font24, "Hewo cweator", (255, 255, 255))
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
            .render_text(font24, "Exit gwame T~T", (255, 255, 255))
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
