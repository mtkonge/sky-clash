use crate::engine::{Context, Error};

use crate::engine::ui::units::*;

pub trait Canvas {
    fn draw_rect(&mut self, size: Size, pos: Pos, color: Rgb) -> Result<(), Error>;
}

impl Canvas for Context<'_, '_> {
    fn draw_rect(&mut self, size: Size, pos: Pos, color: Rgb) -> Result<(), Error> {
        let Size(w, h) = size;
        let Pos(x, y) = pos;
        let Rgb(r, g, b) = color;
        self.draw_rect((r, g, b), x, y, w, h)
    }
}
