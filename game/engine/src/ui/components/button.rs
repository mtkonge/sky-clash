use crate::{font::Font, ui::prelude::Widget, Context, Id, Text as EngineText};

use super::super::units::*;

pub struct Button {
    size: Size,
    text: Text,
}

pub struct Text(pub EngineText);

impl Button {
    pub fn new<S: Into<Size>, T: Into<Text>>(size: S, text: T) -> Self {
        Button {
            size: size.into(),
            text: text.into(),
        }
    }
}

impl Widget for Button {
    fn render(
        &self,
        position: Offset,
        canvas: &mut dyn crate::ui::canvas::Canvas,
    ) -> Result<(), crate::Error> {
        let Size(text_x, text_y) = (self.size - self.text.size()) / 2;
        let text_padding = Offset(text_x as i32, text_y as i32);
        canvas.draw_rect(self.size, position, Rgb(200, 200, 200))?;
        self.text.render(position + text_padding, canvas)?;
        Ok(())
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl Text {
    pub fn new(text: EngineText) -> Self {
        Self(text)
    }
}

impl Widget for Text {
    fn render(
        &self,
        offset: Offset,
        canvas: &mut dyn crate::ui::canvas::Canvas,
    ) -> Result<(), crate::Error> {
        canvas.draw_text(self, offset)
    }

    fn size(&self) -> Size {
        let (w, h) = self.0.size;
        Size(w as u32, h as u32)
    }
}
