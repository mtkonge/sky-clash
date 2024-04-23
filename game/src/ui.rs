use sdl2::keyboard::Keycode;

use crate::engine::{ui::prelude::*, System};
use crate::{query, spawn};

pub struct Menu0(pub u64);
impl System for Menu0 {
    fn on_add(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
        let root = WidgetRc::new(VerticallyCentered::from_children(vec![
            HorizontallyCentered::from_children(vec![
                Rect::from_size((300, 300)).into(),
                Rect::from_size((200, 200)).into(),
            ])
            .into(),
            Rect::from_size((200, 200)).into(),
            Rect::from_size((150, 150)).into(),
        ]))
        .with_id(self.0);
        spawn!(ctx, root);
        Ok(())
    }
    fn on_update(
        &self,
        ctx: &mut crate::engine::Context,
        _delta: f64,
    ) -> Result<(), crate::engine::Error> {
        for id in query!(ctx, WidgetRc) {
            let widget = ctx.entity_component::<WidgetRc>(id).clone();
            widget.render(Pos(0, 0), ctx)?;
        }
        if ctx.key_pressed(Keycode::J) {
            ctx.add_system(|id| Menu1(id));
            ctx.remove_system(self.0);
        }
        Ok(())
    }
    fn on_remove(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
        for id in query!(ctx, WidgetRc) {
            let widget = ctx.entity_component::<WidgetRc>(id);
            if widget.creator_id.is_some_and(|v| v == self.0) {
                ctx.despawn(id);
            }
        }
        Ok(())
    }
}

pub struct Menu1(pub u64);
impl System for Menu1 {
    fn on_add(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
        println!("menu1 added id -> {}", self.0);
        let root = WidgetRc::new(VerticallyCentered::from_children(vec![
            HorizontallyCentered::from_children(vec![
                Rect::from_size((100, 300)).into(),
                Rect::from_size((250, 600)).into(),
            ])
            .into(),
            Rect::from_size((50, 900)).into(),
            Rect::from_size((120, 100)).into(),
        ]))
        .with_id(self.0);
        spawn!(ctx, root);
        Ok(())
    }
    fn on_update(
        &self,
        ctx: &mut crate::engine::Context,
        _delta: f64,
    ) -> Result<(), crate::engine::Error> {
        for id in query!(ctx, WidgetRc) {
            let widget = ctx.entity_component::<WidgetRc>(id).clone();
            widget.render(Pos(0, 0), ctx)?;
        }
        if ctx.key_pressed(Keycode::K) {
            ctx.add_system(|id| Menu0(id));
            ctx.remove_system(self.0);
        }
        Ok(())
    }
    fn on_remove(&self, ctx: &mut crate::engine::Context) -> Result<(), crate::engine::Error> {
        for id in query!(ctx, WidgetRc) {
            let widget = ctx.entity_component::<WidgetRc>(id).clone();
            if widget.creator_id.is_some_and(|v| v == self.0) {
                ctx.despawn(id);
            }
        }
        Ok(())
    }
}
