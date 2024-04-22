use crate::{engine::Component, ui::UIComponent};

use super::{
    engine::{self, Id, System},
    ui,
};
use crate::{hero_creator::HeroCreatorSystem, query, spawn};

#[derive(Component, Clone)]
pub struct CleanerUpper {
    system_id: Id,
    systems: Vec<Id>,
    components: Vec<Id>,
}

pub struct MainMenu(pub Id);

impl System for MainMenu {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui::Kind::*;
        let mut dom = ui::Node::new(Vertical(vec![
            ui::Node::new(Title("SkyClash".to_string()))
                .with_font_size(48)
                .build(),
            ui::Node::new(Button("Start Game".to_string()))
                .with_font_size(24)
                .build(),
            ui::Node::new(Button("Hero Editor".to_string()))
                .with_font_size(24)
                .build(),
            ui::Node::new(Button("Exit Game".to_string()))
                .with_font_size(24)
                .build(),
        ]))
        .with_font("textures/ttf/OpenSans.ttf".to_string())
        .as_head();
        dom.layout(ctx, (0, 0), (1280, 720));
        spawn!(
            ctx,
            ui::UIComponent {
                system_id: self.0,
                dom,
                screen_size: (1280, 720),
            },
        );
        Ok(())
    }

    fn on_remove(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        let ui_component_id = query!(ctx, UIComponent)
            .into_iter()
            .filter(|id| ctx.entity_component::<UIComponent>(*id).system_id == self.0)
            .nth(0)
            .unwrap();
        ctx.despawn(ui_component_id);
        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        ctx.draw_rect((0, 0, 0), 0, 0, 1280, 720)?;
        Ok(())
    }
}
