use crate::engine::{Context, Error};

use crate::engine::ui::units::*;

use super::prelude::Text;

pub trait Canvas {
    fn draw_text(&mut self, text: &Text, pos: Offset) -> Result<(), Error>;
    fn draw_rect(&mut self, size: Size, pos: Offset, color: Rgb) -> Result<(), Error>;
}

impl Canvas for Context<'_, '_> {
    fn draw_text(&mut self, text: &Text, pos: Offset) -> Result<(), Error> {
        self.draw_texture(text.0.texture, pos.0, pos.1)
    }
    fn draw_rect(&mut self, size: Size, pos: Offset, color: Rgb) -> Result<(), Error> {
        let Size(w, h) = size;
        let Offset(x, y) = pos;
        let Rgb(r, g, b) = color;
        self.draw_rect((r, g, b), x, y, w, h)
    }
}
