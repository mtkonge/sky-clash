use std::rc::Rc;

use crate::{engine::Component, query, spawn};

use super::{
    engine::{self, System},
    ui,
};

#[derive(Component, Clone)]

pub struct SkillBar {
    pub pos: (i32, i32),
    pub size: (u32, u32),
    pub texture: engine::Texture,
    pub segment: i32,
}

pub struct SkillBarSystem;

impl System for SkillBarSystem {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, SkillBar) {
            let skill_bar = ctx.entity_component::<SkillBar>(id).clone();
            ctx.draw_texture(skill_bar.texture, skill_bar.pos.0, skill_bar.pos.1)?;
        }
        Ok(())
    }
}
pub struct HeroCreator;

impl System for HeroCreator {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        let font48 = ctx.load_font("textures/ttf/OpenSans.ttf", 48)?;
        let font24 = ctx.load_font("textures/ttf/OpenSans.ttf", 24)?;
        ctx.add_system(ui::TitleSystem);
        ctx.add_system(ui::ButtonSystem);

        let texture = ctx
            .render_text(font48, "Hewo cweator", (255, 255, 255))
            .unwrap();
        spawn!(
            ctx,
            ui::Title {
                pos: (400, 100),
                texture
            }
        );
        let texture = ctx.render_text(font24, "+", (255, 255, 255)).unwrap();
        spawn!(
            ctx,
            ui::Button {
                pos: (400, 250),
                size: (400, 80),
                texture,
                action: Rc::new(|_| ())
            },
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        ctx.draw_rect((0, 0, 0), 0, 0, 1280, 720)?;
        Ok(())
    }
}
