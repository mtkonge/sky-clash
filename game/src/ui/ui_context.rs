pub trait UiContext {
    fn draw_rect(
        &mut self,
        rgb: (u8, u8, u8),
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> Result<(), engine::Error>;

    fn draw_texture(
        &mut self,
        texture: engine::Texture,
        x: i32,
        y: i32,
    ) -> Result<(), engine::Error>;

    fn load_font<P>(&mut self, path: P, size: u16) -> Result<engine::Id, engine::Error>
    where
        P: AsRef<std::path::Path>;

    fn render_text<S: Into<String>>(
        &mut self,
        font_id: engine::Id,
        text: S,
        rgb: (u8, u8, u8),
    ) -> Result<engine::Text, engine::Error>;

    fn load_texture<P>(&mut self, path: P) -> Result<engine::Texture, engine::Error>
    where
        P: AsRef<std::path::Path>;

    fn texture_size(&mut self, texture: engine::Texture) -> Result<(u32, u32), engine::Error>;

    fn draw_texture_sized(
        &mut self,
        texture: engine::Texture,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), engine::Error>;

    fn text_size<S: AsRef<str>>(
        &mut self,
        font_id: engine::Id,
        text: S,
    ) -> Result<(u32, u32), engine::Error>;
}

impl UiContext for engine::Context<'_, '_> {
    fn draw_rect(
        &mut self,
        rgb: (u8, u8, u8),
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> Result<(), engine::Error> {
        self.draw_rect(rgb, x, y, w, h)
    }

    fn draw_texture(
        &mut self,
        texture: engine::Texture,
        x: i32,
        y: i32,
    ) -> Result<(), engine::Error> {
        self.draw_texture(texture, x, y)
    }

    fn load_font<P>(&mut self, path: P, size: u16) -> Result<engine::Id, engine::Error>
    where
        P: AsRef<std::path::Path>,
    {
        self.load_font(path, size)
    }

    fn render_text<S: Into<String>>(
        &mut self,
        font_id: engine::Id,
        text: S,
        rgb: (u8, u8, u8),
    ) -> Result<engine::Text, engine::Error> {
        self.render_text(font_id, text, rgb)
    }

    fn load_texture<P>(&mut self, path: P) -> Result<engine::Texture, engine::Error>
    where
        P: AsRef<std::path::Path>,
    {
        self.load_texture(path)
    }

    fn texture_size(&mut self, texture: engine::Texture) -> Result<(u32, u32), engine::Error> {
        self.texture_size(texture)
    }

    fn draw_texture_sized(
        &mut self,
        texture: engine::Texture,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), engine::Error> {
        self.draw_texture_sized(texture, x, y, width, height)
    }

    fn text_size<S: AsRef<str>>(
        &mut self,
        font_id: engine::Id,
        text: S,
    ) -> Result<(u32, u32), engine::Error> {
        self.text_size(font_id, text)
    }
}

pub struct MockContext;

#[allow(unused_variables)]
impl UiContext for MockContext {
    fn draw_rect(
        &mut self,
        rgb: (u8, u8, u8),
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> Result<(), engine::Error> {
        unreachable!()
    }

    fn draw_texture(
        &mut self,
        texture: engine::Texture,
        x: i32,
        y: i32,
    ) -> Result<(), engine::Error> {
        unreachable!()
    }

    fn load_font<P>(&mut self, path: P, size: u16) -> Result<engine::Id, engine::Error>
    where
        P: AsRef<std::path::Path>,
    {
        unreachable!()
    }

    fn render_text<S: Into<String>>(
        &mut self,
        font_id: engine::Id,
        text: S,
        rgb: (u8, u8, u8),
    ) -> Result<engine::Text, engine::Error> {
        unreachable!()
    }

    fn load_texture<P>(&mut self, path: P) -> Result<engine::Texture, engine::Error>
    where
        P: AsRef<std::path::Path>,
    {
        unreachable!()
    }

    fn texture_size(&mut self, texture: engine::Texture) -> Result<(u32, u32), engine::Error> {
        unreachable!()
    }

    fn draw_texture_sized(
        &mut self,
        texture: engine::Texture,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), engine::Error> {
        unreachable!()
    }

    fn text_size<S: AsRef<str>>(
        &mut self,
        font_id: engine::Id,
        text: S,
    ) -> Result<(u32, u32), engine::Error> {
        unreachable!()
    }
}
