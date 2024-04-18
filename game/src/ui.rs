use super::engine::{self, Component, System};
use crate::query;
use std::rc::Rc;

#[derive(Component, Clone)]
pub struct Title {
    pub pos: (i32, i32),
    pub texture: engine::Texture,
}

pub struct TitleSystem;
impl System for TitleSystem {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, Title) {
            let title = ctx.entity_component::<Title>(id).clone();
            ctx.draw_texture(title.texture, title.pos.0, title.pos.1)?;
        }
        Ok(())
    }
}

#[derive(Component, Clone)]
pub struct Button {
    pub pos: (i32, i32),
    pub size: (u32, u32),
    pub texture: engine::Texture,
    pub action: Rc<dyn Fn(&mut engine::Context)>,
}

impl Button {
    fn contains(&self, mouse_pos: (i32, i32)) -> bool {
        (self.pos.0..=self.pos.0 + self.size.0 as i32).contains(&mouse_pos.0)
            && (self.pos.1..=self.pos.1 + self.size.1 as i32).contains(&mouse_pos.1)
    }
}

pub struct ButtonSystem;
impl System for ButtonSystem {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, Button) {
            let button = ctx.entity_component::<Button>(id).clone();
            let position = ctx.mouse_position();
            ctx.draw_rect(
                (255, 255, 255),
                button.pos.0,
                button.pos.1,
                button.size.0,
                button.size.1,
            )?;
            if button.contains(position) {
                ctx.draw_rect(
                    (40, 40, 40),
                    button.pos.0 + 1,
                    button.pos.1 + 1,
                    button.size.0 - 2,
                    button.size.1 - 2,
                )?;
            } else {
                ctx.draw_rect(
                    (0, 0, 0),
                    button.pos.0 + 1,
                    button.pos.1 + 1,
                    button.size.0 - 2,
                    button.size.1 - 2,
                )?;
            }
            ctx.draw_texture(button.texture, button.pos.0 + 3, button.pos.1 + 3)?;
            if button.contains(position) && ctx.mouse_button_pressed(engine::MouseButton::Left) {
                (button.action)(ctx);
            }
        }
        Ok(())
    }
}
