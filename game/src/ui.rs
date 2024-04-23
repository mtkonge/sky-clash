use crate::engine::{ui::prelude::*, System};

pub struct UI;
impl System for UI {
    fn on_update(
        &self,
        ctx: &mut crate::engine::Context,
        _delta: f64,
    ) -> Result<(), crate::engine::Error> {
        let root = VerticallyCentered::from_children(vec![
            HorizontallyCentered::from_children(vec![
                Rect::from_size((300, 300)).into(),
                Rect::from_size((200, 200)).into(),
            ])
            .into(),
            Rect::from_size((200, 200)).into(),
            Rect::from_size((150, 150)).into(),
        ]);
        root.render(Pos(0, 0), ctx)?;
        Ok(())
    }
}
