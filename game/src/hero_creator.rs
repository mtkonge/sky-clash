use crate::{
    engine::{Component, Id},
    query, spawn,
};

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
pub struct HeroCreator(pub Id);

impl System for HeroCreator {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        let font48 = ctx.load_font("textures/ttf/OpenSans.ttf", 48)?;
        let font24 = ctx.load_font("textures/ttf/OpenSans.ttf", 24)?;
        ctx.add_system(ui::TitleSystem);
        ctx.add_system(ui::ButtonSystem);

        let text = ctx
            .render_text(font48, "Hewo cweator", (255, 255, 255))
            .unwrap();

        spawn!(
            ctx,
            ui::Title {
                pos: (400, 100),
                texture: text.texture
            }
        );

        let text = ctx.render_text(font48, "0", (255, 255, 255)).unwrap();

        spawn!(
            ctx,
            ui::Title {
                pos: (400, 200),
                texture: text.texture
            }
        );

        let text = ctx.render_text(font48, "0", (255, 255, 255)).unwrap();

        spawn!(
            ctx,
            ui::Title {
                pos: (400, 250),
                texture: text.texture
            }
        );

        let text = ctx.render_text(font48, "0", (255, 255, 255)).unwrap();

        spawn!(
            ctx,
            ui::Title {
                pos: (400, 300),
                texture: text.texture
            }
        );

        let text = ctx.render_text(font24, "+", (130, 255, 130)).unwrap();

        spawn!(
            ctx,
            ui::Button::new((450, 250), (50, 50)).with_centered_text(text)
        );

        let text = ctx.render_text(font24, "-", (255, 130, 130)).unwrap();

        spawn!(
            ctx,
            ui::Button::new((500, 250), (50, 50)).with_centered_text(text)
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        ctx.draw_rect((0, 0, 0), 0, 0, 1280, 720)?;
        Ok(())
    }
}
