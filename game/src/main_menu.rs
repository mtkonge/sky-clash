use crate::engine::Component;

use super::{
    engine::{self, Id, System},
    ui,
};
use crate::{hero_creator::HeroCreator, query, spawn};

#[derive(Component, Clone)]
pub struct CleanerUpper {
    system_id: Id,
    systems: Vec<Id>,
    components: Vec<Id>,
}

pub struct MainMenu(pub Id);

impl System for MainMenu {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        let system_id = self.0;
        let font48 = ctx.load_font("textures/ttf/OpenSans.ttf", 48)?;
        let font24 = ctx.load_font("textures/ttf/OpenSans.ttf", 24)?;
        let title_system_id = ctx.add_system(ui::TitleSystem);
        let button_system_id = ctx.add_system(ui::ButtonSystem);
        let systems = vec![title_system_id, button_system_id];
        let text = ctx
            .render_text(font48, "SkyCwash", (255, 255, 255))
            .unwrap();
        let c0 = spawn!(
            ctx,
            ui::Title {
                pos: (400, 100),
                texture: text.texture,
            }
        );
        let text = ctx
            .render_text(font24, "Pway gwame", (255, 255, 255))
            .unwrap();

        let c1 = spawn!(
            ctx,
            ui::Button::new((400, 250), (400, 80)).with_centered_text(text)
        );

        let text = ctx
            .render_text(font24, "Hewo cweator", (255, 255, 255))
            .unwrap();
        let c3 = spawn!(
            ctx,
            ui::Button::new((400, 350), (400, 80))
                .with_centered_text(text)
                .with_action(move |ctx| {
                    ctx.remove_system(system_id);
                    ctx.add_system(HeroCreator);
                })
        );

        let text = ctx
            .render_text(font24, "Exit gwame T~T", (255, 255, 255))
            .unwrap();
        let c2 = spawn!(
            ctx,
            ui::Button::new((400, 450), (400, 80))
                .with_centered_text(text)
                .with_action(|_| panic!())
        );

        let components = vec![c0, c1, c2, c3];
        spawn!(
            ctx,
            CleanerUpper {
                system_id: self.0,
                systems,
                components
            }
        );

        Ok(())
    }

    fn on_remove(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        let cleaner = query!(ctx, CleanerUpper)
            .iter()
            .find(|id| ctx.entity_component::<CleanerUpper>(**id).system_id == self.0)
            .map(|id| ctx.entity_component::<CleanerUpper>(*id))
            .unwrap()
            .clone();
        for id in cleaner.systems {
            ctx.remove_system(id);
        }
        for id in cleaner.components {
            ctx.despawn(id);
        }
        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        ctx.draw_rect((0, 0, 0), 0, 0, 1280, 720)?;
        Ok(())
    }
}
