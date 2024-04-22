use std::{fmt::format, rc::Rc};

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

#[derive(Component, Clone)]
pub struct Skills {
    pub strength: i32,
    pub agility: i32,
    pub defence: i32,
}

impl Skills {
    pub fn new() -> Self {
        Self {
            strength: 0,
            agility: 0,
            defence: 0,
        }
    }
}

#[derive(Component, Clone)]
pub struct HeroCreator {
    pub system_id: Id,
    pub skills_id: Id,
    pub strength_title: Id,
    pub agility_title: Id,
    pub defence_title: Id,
}

pub struct HeroCreatorSystem(pub Id);

impl System for HeroCreatorSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        let font48 = ctx.load_font("textures/ttf/OpenSans.ttf", 48)?;
        let font24 = ctx.load_font("textures/ttf/OpenSans.ttf", 24)?;
        ctx.add_system(ui::TitleSystem);
        ctx.add_system(ui::ButtonSystem);

        let skills_id = spawn!(ctx, Skills::new());
        let Skills {
            strength,
            agility,
            defence,
        } = ctx.entity_component::<Skills>(skills_id).clone();

        let text = ctx
            .render_text(font48, "Hewo cweator", (255, 255, 255))
            .unwrap();

        let strength_title = spawn!(
            ctx,
            ui::Title {
                pos: (400, 100),
                texture: text.texture
            }
        );

        let text = ctx
            .render_text(font48, &format!("Strength: {strength}"), (255, 255, 255))
            .unwrap();

        spawn!(
            ctx,
            ui::Title {
                pos: (400, 200),
                texture: text.texture
            }
        );

        let text = ctx.render_text(font24, "+", (130, 255, 130)).unwrap();

        spawn!(
            ctx,
            ui::Button::new((650, 200 + 10), (50, 50))
                .with_centered_text(text)
                .with_action(move |ctx| {
                    let skills = ctx.entity_component::<Skills>(skills_id);
                    skills.strength -= 1;
                }),
        );

        let text = ctx.render_text(font24, "-", (255, 130, 130)).unwrap();

        spawn!(
            ctx,
            ui::Button::new((700, 200 + 10), (50, 50))
                .with_centered_text(text)
                .with_action(move |ctx| {
                    let skills = ctx.entity_component::<Skills>(skills_id);
                    skills.strength += 1;
                }),
        );

        let text = ctx
            .render_text(font48, &format!("Agility: {agility}"), (255, 255, 255))
            .unwrap();

        let agility_title = spawn!(
            ctx,
            ui::Title {
                pos: (400, 250),
                texture: text.texture
            }
        );

        let text = ctx.render_text(font24, "+", (130, 255, 130)).unwrap();

        spawn!(
            ctx,
            ui::Button::new((650, 250 + 10), (50, 50))
                .with_centered_text(text)
                .with_action(move |ctx| {
                    let skills = ctx.entity_component::<Skills>(skills_id);
                    skills.agility -= 1;
                }),
        );

        let text = ctx.render_text(font24, "-", (255, 130, 130)).unwrap();

        spawn!(
            ctx,
            ui::Button::new((700, 250 + 10), (50, 50))
                .with_centered_text(text)
                .with_action(move |ctx| {
                    let skills = ctx.entity_component::<Skills>(skills_id);
                    skills.agility -= 1;
                }),
        );

        let text = ctx
            .render_text(font48, &format!("Defence: {defence}"), (255, 255, 255))
            .unwrap();

        let defence_title = spawn!(
            ctx,
            ui::Title {
                pos: (400, 300),
                texture: text.texture
            }
        );

        let text = ctx.render_text(font24, "+", (130, 255, 130)).unwrap();

        spawn!(
            ctx,
            ui::Button::new((650, 300 + 10), (50, 50))
                .with_centered_text(text)
                .with_action(move |ctx| {
                    let skills = ctx.entity_component::<Skills>(skills_id);
                    skills.defence -= 1;
                }),
        );

        let text = ctx.render_text(font24, "-", (255, 130, 130)).unwrap();

        spawn!(
            ctx,
            ui::Button::new((700, 300 + 10), (50, 50))
                .with_centered_text(text)
                .with_action(move |ctx| {
                    let skills = ctx.entity_component::<Skills>(skills_id);
                    skills.defence -= 1;
                }),
        );

        spawn!(
            ctx,
            HeroCreator {
                system_id: self.0,
                skills_id,
                strength_title,
                agility_title,
                defence_title
            },
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        let creator = query!(ctx, HeroCreator)
            .into_iter()
            .filter_map(|id| Some(ctx.entity_component::<HeroCreator>(id).clone()))
            .nth(0)
            .unwrap();
        let skills = query!(ctx, Skills)
            .into_iter()
            .filter_map(|id| Some(ctx.entity_component::<Skills>(creator.skills_id).clone()));
        let strength_title = query!(ctx, Skills)
            .into_iter()
            .filter_map(|id| Some(ctx.entity_component::<Skills>(creator.skills_id).clone()));

        let text = ctx
            .render_text(font48, "Hewo cweator", (255, 255, 255))
            .unwrap();

        ctx.draw_rect((0, 0, 0), 0, 0, 1280, 720)?;
        Ok(())
    }
}
