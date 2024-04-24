use sdl2::keyboard::Keycode;

use crate::engine::{ui::prelude::*, System};
use crate::{query, spawn};

enum KewlEvent {
    Cwick,
}

pub struct Menu0(pub u64);
impl System for Menu0 {
    fn on_add(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
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
            let widget = ctx.entity_component::<Root>(id).clone();
            widget.render(Offset(0, 0), ctx)?;
        }
        if ctx.key_pressed(Keycode::J) {
            ctx.add_system(|id| Menu1(id));
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

pub struct Menu1(pub u64);
impl System for Menu1 {
    fn on_add(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
        let font = ctx.load_font("textures/ttf/OpenSans.ttf", 24)?;
        let text = ctx.render_text(font, "hewwo", (255, 255, 255))?;

        let root = Root::new(
            self.0,
            VerticallyCentered::from_children(vec![HorizontallyCentered::from_children(vec![
                Rect::from_size((100, 300)).into(),
                Button::new((300, 300), Text::new(text)).into(),
            ])
            .into()]),
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
            let widget = ctx.entity_component::<Root>(id).clone();
            widget.render(Offset(0, 0), ctx)?;
        }
        if ctx.key_pressed(Keycode::K) {
            ctx.add_system(|id| Menu0(id));
            ctx.remove_system(self.0);
        }
        Ok(())
    }
    fn on_remove(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
        for id in query!(ctx, Root) {
            let widget = ctx.entity_component::<Root>(id).clone();
            if widget.creator_id == self.0 {
                ctx.despawn(id);
            }
        }
        Ok(())
    }
}
