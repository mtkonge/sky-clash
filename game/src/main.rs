#![allow(dead_code)]

mod engine;

use engine::{Component, System};
use std::rc::Rc;

#[derive(Component, Clone)]
struct Title {
    pos: (i32, i32),
    texture: engine::Texture,
}

struct TitleSystem;
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
struct Button {
    pos: (i32, i32),
    size: (u32, u32),
    texture: engine::Texture,
    action: Rc<dyn Fn(&mut engine::Context)>,
}

impl Button {
    fn contains(&self, mouse_pos: (i32, i32)) -> bool {
        (self.pos.0..=self.pos.0 + self.size.0 as i32).contains(&mouse_pos.0)
            && (self.pos.1..=self.pos.1 + self.size.1 as i32).contains(&mouse_pos.1)
    }
}

struct ButtonSystem;
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

struct MainMenu;
impl System for MainMenu {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        ctx.add_system(TitleSystem);
        ctx.add_system(ButtonSystem);
        let texture = ctx
            .render_text("textures/ttf/OpenSans.ttf", "SkyCwash", 48, (255, 255, 255))
            .unwrap();
        spawn!(
            ctx,
            Title {
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
            Button {
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
            Button {
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
            Button {
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

fn main() {
    let mut game = engine::Game::new().unwrap();

    let mut ctx = game.context();
    ctx.add_system(MainMenu);
    // context.add_system(MenuSystem);

    game.run();
}
