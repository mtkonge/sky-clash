use crate::engine::{
    ui::{
        canvas::Canvas,
        widget::{Widget, WithPos, WithSize},
    },
    Error,
};

use crate::engine::ui::units::*;

#[derive(Default)]
pub struct Rect {
    color: Rgb,
    size: Size,
    pos: Offset,
}

impl WithPos for Rect {
    fn with_pos<T: Into<Offset>>(mut self, pos: T) -> Self {
        self.pos = pos.into();
        self
    }
}

impl WithSize for Rect {
    fn with_size<T: Into<Size>>(mut self, size: T) -> Self {
        self.size = size.into();
        self
    }
}

impl Rect {
    pub fn with_pos(mut self, pos: Offset) -> Self {
        self.pos = pos;
        self
    }
    pub fn with_size<T: Into<Size>>(mut self, size: T) -> Self {
        self.size = size.into();
        self
    }
}

impl Widget for Rect {
    fn render(&self, pos: Offset, canvas: &mut dyn Canvas) -> Result<(), Error> {
        canvas.draw_rect(
            self.size.clone(),
            pos + self.pos.clone(),
            self.color.clone(),
        )
    }

    fn size(&self) -> Size {
        self.size
    }
}
