use super::engine::{self, Component, System};
use crate::{engine::Id, query};
use std::rc::Rc;

#[derive(Component, Clone)]
pub struct Title {
    pub pos: (i32, i32),
    pub texture: engine::Texture,
}

pub struct TitleSystem(pub Id);
impl System for TitleSystem {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, Title) {
            let title = ctx.entity_component::<Title>(id).clone();
            ctx.draw_texture(title.texture, title.pos.0, title.pos.1)?;
        }
        Ok(())
    }
}

type Action = dyn Fn(&mut engine::Context);

#[derive(Component, Clone, Default)]
pub struct Button {
    pub pos: (i32, i32),
    pub size: (u32, u32),
    pub text: Option<engine::Text>,
    pub texture_offset: Option<(i32, i32)>,
    pub action: Option<Rc<Action>>,
}

impl Button {
    pub fn new(pos: (i32, i32), size: (u32, u32)) -> Self {
        Self {
            pos,
            size,
            ..Default::default()
        }
    }
    pub fn with_centered_text(mut self, text: engine::Text) -> Self {
        let button_size = (self.size.0 as i32, self.size.1 as i32);
        let text_size = text.size;
        self.text = Some(text);
        self.texture_offset = Some((
            (button_size.0 - text_size.0) / 2,
            (button_size.1 - text_size.1) / 2,
        ));
        self
    }
    pub fn with_texture_and_offset(
        mut self,
        text: engine::Text,
        texture_offset: (i32, i32),
    ) -> Self {
        self.text = Some(text);
        self.texture_offset = Some(texture_offset);
        self
    }
    pub fn with_action<A: 'static + Fn(&mut engine::Context)>(mut self, action: A) -> Self {
        self.action = Some(Rc::new(action));
        self
    }

    fn contains(&self, mouse_pos: (i32, i32)) -> bool {
        (self.pos.0..=self.pos.0 + self.size.0 as i32).contains(&mouse_pos.0)
            && (self.pos.1..=self.pos.1 + self.size.1 as i32).contains(&mouse_pos.1)
    }
}

pub struct ButtonSystem(pub Id);
impl System for ButtonSystem {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        let mut actions = Vec::new();

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
            if let Some(text) = button.text {
                let offset = button.texture_offset.unwrap_or_default();
                ctx.draw_texture(
                    text.texture,
                    button.pos.0 + offset.0,
                    button.pos.1 + offset.1,
                )?;
            }
            if button.contains(position) && ctx.mouse_button_pressed(engine::MouseButton::Left) {
                if let Some(action) = button.action.clone() {
                    actions.push(action);
                }
            }
        }
        for action in actions {
            action(ctx)
        }
        Ok(())
    }
}
