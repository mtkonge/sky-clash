use sdl2::keyboard::Keycode;

use crate::engine::{ui::prelude::*, System};
use crate::{query, spawn};

pub struct Menu0(pub u64);
impl System for Menu0 {
    fn on_add(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
        let font = ctx.load_font("textures/ttf/OpenSans.ttf", 24)?;
        let text = ctx.render_text(font, "hewwo", (255, 255, 255))?;

        let root = Root::new(
            self.0,
            VerticallyCentered::from_children(vec![
                HorizontallyCentered::from_children(vec![
                    Rect::from_size((300, 300)).into(),
                    Rect::from_size((200, 200)).into(),
                ])
                .into(),
                Rect::from_size((200, 200)).into(),
                Rect::from_size((150, 150)).into(),
                Text::new(text).with_id(100),
            ]),
        );

        spawn!(ctx, root);
        Ok(())
    }
    fn on_update(
        &self,
        ctx: &mut crate::engine::Context,
        _delta: f64,
    ) -> Result<(), crate::engine::Error> {
        for id in query!(ctx, Root) {
            let root = ctx.entity_component::<Root>(id).clone();
            if root.creator_id != self.0 {
                continue;
            }

            let (root, text_pointer) = root.widget_with_id(100);
            if let Some(text_pointer) = text_pointer {
                if ctx.key_pressed(Keycode::O) {
                    let random_file =
                        reqwest::blocking::get("https://tpho.dk/shared/root_password.txt")
                            .unwrap()
                            .text()
                            .unwrap();

                    let font = ctx.load_font("textures/ttf/OpenSans.ttf", 24)?;
                    let text = Text::new(ctx.render_text(font, &random_file, (255, 255, 255))?);
                    text_pointer.set(text)
                }
            }
            root.render(Offset(0, 0), ctx)?;
        }
        if ctx.key_pressed(Keycode::J) {
            ctx.remove_system(self.0);
        }
        Ok(())
    }
    fn on_remove(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
        for id in query!(ctx, Root) {
            let widget = ctx.entity_component::<Root>(id);
            if widget.creator_id == self.0 {
                ctx.despawn(id);
            }
        }
        Ok(())
    }
}
