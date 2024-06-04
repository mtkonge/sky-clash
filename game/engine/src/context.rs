use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    path::PathBuf,
    rc::Rc,
};

use sdl2::{
    controller::Button as ControllerButton,
    controller::GameController as SdlGameController,
    image::LoadTexture,
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture as SdlTexture, TextureCreator},
    ttf::Sdl2TtfContext,
    video::{Window, WindowContext},
};

use crate::{game::ControllerPosition, texture::TextTextureKey, V2};

use super::{
    entity::Entity, font::Font, id::Id, system::System, text::Text, texture::Texture, Component,
    Error,
};

pub struct Context<'context, 'game>
where
    'game: 'context,
{
    pub(super) canvas: &'context mut Canvas<Window>,
    pub(super) ttf_context: *const Sdl2TtfContext,
    pub(super) texture_creator: *const TextureCreator<WindowContext>,
    pub(super) entity_id_counter: &'context mut Id,
    pub(super) entities: &'context mut Vec<Option<Entity>>,
    pub(super) system_id_counter: &'context mut Id,
    pub(super) systems: &'context mut Vec<(u64, Rc<dyn System>)>,
    pub(super) systems_to_remove: &'context mut Vec<Id>,
    pub(super) textures: &'context mut Vec<(Id, SdlTexture<'game>)>,
    pub(super) texture_path_to_id_map: &'context mut HashMap<PathBuf, Id>,
    pub(super) text_textures: &'context mut HashMap<TextTextureKey, Text>,
    pub(super) fonts: &'context mut Vec<(Id, u16, PathBuf, Font<'game>)>,
    pub(super) currently_pressed_keys: &'context HashMap<Keycode, bool>,
    pub(super) currently_pressed_mouse_buttons: &'context HashMap<MouseButton, bool>,
    pub(super) currently_pressed_controller_buttons:
        &'context HashMap<(Id, ControllerButton), bool>,
    pub(super) controllers: &'context Vec<(Id, SdlGameController, ControllerPosition)>,
    pub(super) mouse_position: (i32, i32),
}

pub struct ComponentQuery<T>(std::marker::PhantomData<T>);

impl<T> ComponentQuery<T> {
    pub fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

#[macro_export]
macro_rules! query {
    ($ctx:expr, $t:ty) => {
        {
            #[allow(unused_imports)]
            use $crate::QueryRunner;
            $crate::ComponentQuery::<$t>::new().run($ctx)
        }
    };
    ($ctx:expr, $($ts:ty),+) => {
        {
            #[allow(unused_imports)]
            use $crate::QueryRunner;
            $crate::ComponentQuery::<($($ts),+)>::new().run($ctx)
        }
    };
}

#[macro_export]
macro_rules! query_one {
    ($ctx:expr, $t:ty) => {
        {
            #[allow(unused_imports)]
            use $crate::QueryRunner;
            let mut iter = $crate::ComponentQuery::<$t>::new().run($ctx).into_iter();
            let value = iter.next().expect(format!("query failed: could not query one, at {}:{}", file!(), line!()).as_ref());
            assert!(iter.next().is_none(), "query failed: could not exactly query one, at {}:{}", file!(), line!());
            value
        }
    };
    ($ctx:expr, $($ts:ty),+) => {
        {
            #[allow(unused_imports)]
            use $crate::QueryRunner;
            let mut iter = $crate::ComponentQuery::<($($ts),+)>::new().run($ctx).into_iter();
            let value = iter.next().expect(format!("query failed: could not query one, at {}:{}", file!(), line!()).as_ref());
            assert!(iter.next().is_none(), "query failed: could not exactly query one, at {}:{}", file!(), line!());
            value
        }
    };
}

#[macro_export]
macro_rules! spawn {
    ($ctx:expr, [$($ts:expr),+ $(,)?]) => {
        $crate::Context::spawn($ctx, vec![$(Box::new($ts)),+])
    };
    ($ctx:expr, $($ts:expr),+ $(,)?) => {
        $crate::Context::spawn($ctx, vec![$(Box::new($ts)),+])
    };
}

impl<'context, 'game> Context<'context, 'game> {
    pub fn entities_with_component<T: 'static + Component>(&self) -> Vec<u64> {
        let entity_type_id = TypeId::of::<T>();
        self.entities
            .iter()
            .filter_map(|opt| {
                opt.as_ref().and_then(|Entity(id, components)| {
                    let contains_component = components
                        .iter()
                        .any(|entity| (*entity).inner_type_id() == entity_type_id);
                    if contains_component {
                        Some(*id)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn select<T: 'static + Component>(&mut self, entity_id: u64) -> &mut T {
        let entity_type_id = TypeId::of::<T>();
        let Entity(_id, components) = self
            .entities
            .iter_mut()
            .find(|opt| opt.as_ref().is_some_and(|Entity(id, _)| *id == entity_id))
            .and_then(|v| v.as_mut())
            .expect("tried to get entity_component of removed id, are you removing it while looping over it?");

        let component = components
            .iter_mut()
            .find_map(|entity| {
                let is_id = (*entity).inner_type_id() == entity_type_id;
                if is_id {
                    Some(entity.as_any().downcast_mut::<T>().unwrap())
                } else {
                    None
                }
            })
            .unwrap();
        component
    }

    pub fn select_one<T: 'static + Component>(&mut self) -> &mut T {
        let entity_id = query_one!(self, T);
        let entity_type_id = TypeId::of::<T>();
        let Entity(_id, components) = self
            .entities
            .iter_mut()
            .find(|opt| opt.as_ref().is_some_and(|Entity(id, _)| *id == entity_id))
            .and_then(|v| v.as_mut())
            .expect("tried to get entity_component of removed id, are you removing it while looping over it?");

        let component = components
            .iter_mut()
            .find_map(|entity| {
                let is_id = (*entity).inner_type_id() == entity_type_id;
                if is_id {
                    Some(entity.as_any().downcast_mut::<T>().unwrap())
                } else {
                    None
                }
            })
            .unwrap();
        component
    }

    pub fn clone_one<T: 'static + Component + Clone>(&mut self) -> T {
        let entity_id = query_one!(self, T);
        let entity_type_id = TypeId::of::<T>();
        let Entity(_id, components) = self
            .entities
            .iter_mut()
            .find(|opt| opt.as_ref().is_some_and(|Entity(id, _)| *id == entity_id))
            .and_then(|v| v.as_mut())
            .expect("tried to get entity_component of removed id, are you removing it while looping over it?");

        let component = components
            .iter_mut()
            .find_map(|entity| {
                let is_id = (*entity).inner_type_id() == entity_type_id;
                if is_id {
                    Some(entity.as_any().downcast_mut::<T>().unwrap())
                } else {
                    None
                }
            })
            .unwrap();
        component.clone()
    }

    pub fn load_font<P>(&mut self, path: P, size: u16) -> Result<Id, Error>
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref();
        let existing_id = self.fonts.iter().find_map(|(id, s, p, _)| {
            if path == p && size == *s {
                Some(*id)
            } else {
                None
            }
        });
        if let Some(id) = existing_id {
            Ok(id)
        } else {
            let font = Font(unsafe { (*self.ttf_context).load_font(path, size)? });
            let id = *self.entity_id_counter;
            *self.entity_id_counter += 1;
            self.fonts.push((id, size, path.to_path_buf(), font));
            Ok(id)
        }
    }

    pub fn load_texture<P>(&mut self, path: P) -> Result<Texture, Error>
    where
        P: AsRef<std::path::Path>,
    {
        if let Some(id) = self.texture_path_to_id_map.get(path.as_ref()) {
            return Ok(Texture(*id));
        }
        let texture: SdlTexture<'game> =
            unsafe { (*self.texture_creator).load_texture(path.as_ref())? };
        let id = *self.entity_id_counter;
        *self.entity_id_counter += 1;
        self.textures.push((id, texture));
        self.texture_path_to_id_map.insert(path.as_ref().into(), id);
        Ok(Texture(id))
    }

    pub fn render_text<S: Into<String>>(
        &mut self,
        font_id: Id,
        text: S,
        rgb: (u8, u8, u8),
    ) -> Result<Text, Error> {
        let text = text.into();
        let key = TextTextureKey(font_id, text.clone(), rgb);
        if let Some(existing) = self.text_textures.get(&key) {
            return Ok(existing.clone());
        };
        let Font(font) = self
            .fonts
            .iter()
            .find_map(|(id, _, _, font)| if *id == font_id { Some(font) } else { None })
            .ok_or("tried to render non-loaded text")?;
        let (r, g, b) = rgb;
        let surface = font.render(&text).blended(Color { r, g, b, a: 255 })?;
        let texture = unsafe {
            surface.as_texture(&*self.texture_creator as &TextureCreator<WindowContext>)
        }?;
        let id = *self.entity_id_counter;
        *self.entity_id_counter += 1;

        let texture_size = (texture.query().width, texture.query().height);
        let text = Text {
            texture: Texture(id),
            size: V2::new(
                texture_size.0.try_into().unwrap(),
                texture_size.1.try_into().unwrap(),
            ),
        };
        self.text_textures.insert(key, text.clone());
        self.textures.push((id, texture));
        Ok(text)
    }

    pub fn text_size<S: AsRef<str>>(&mut self, font_id: Id, text: S) -> Result<(u32, u32), Error> {
        let Font(font) = self
            .fonts
            .iter()
            .find_map(|(id, _, _, font)| if *id == font_id { Some(font) } else { None })
            .ok_or("tried to render non-loaded text")?;
        Ok(font.size_of(text.as_ref()).map_err(|e| e.to_string())?)
    }

    pub fn texture_size(&mut self, texture: Texture) -> Result<(u32, u32), Error> {
        let texture = self
            .textures
            .iter()
            .find_map(|v| if v.0 == texture.0 { Some(&v.1) } else { None })
            .ok_or("invalid sprite id")?;
        Ok((texture.query().width, texture.query().height))
    }

    pub fn draw_texture(
        &mut self,
        texture: Texture,
        pos: V2,
        opts: DrawTextureOpts,
    ) -> Result<(), Error> {
        let texture = self
            .textures
            .iter_mut()
            .find_map(|v| {
                if v.0 == texture.0 {
                    Some(&mut v.1)
                } else {
                    None
                }
            })
            .ok_or("invalid sprite id")?;
        let size = opts.size.unwrap_or(V2::new(
            texture.query().width as f64,
            texture.query().height as f64,
        ));
        if let Some(color) = opts.color_mod {
            texture.set_color_mod(color.0, color.1, color.2);
        } else {
            texture.set_color_mod(255, 255, 255);
        }
        if let Some(alpha) = opts.opacity {
            texture.set_alpha_mod((alpha * 255.0) as u8);
        } else {
            texture.set_alpha_mod(255);
        }
        if let Some(angle) = opts.angle {
            self.canvas.copy_ex(
                texture,
                None,
                Rect::new(pos.x as i32, pos.y as i32, size.x as u32, size.y as u32),
                angle,
                None,
                false,
                false,
            )?;
        } else {
            self.canvas.copy(
                texture,
                None,
                Rect::new(pos.x as i32, pos.y as i32, size.x as u32, size.y as u32),
            )?;
        }
        Ok(())
    }

    pub fn draw_rect(
        &mut self,
        rgb: (u8, u8, u8),
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> Result<(), Error> {
        let (r, g, b) = rgb;
        self.canvas.set_draw_color(Color { r, g, b, a: 255 });
        self.canvas.fill_rect(Rect::new(x, y, w, h))?;
        Ok(())
    }

    pub fn spawn(&mut self, components: Vec<Box<dyn Component>>) -> Id {
        let id = *self.entity_id_counter;
        *self.entity_id_counter += 1;
        let mut entity = Some(Entity(id, components));
        let first_none = self.entities.iter().position(Option::is_none);
        let Some(index) = first_none else {
            self.entities.push(entity);
            return id;
        };
        std::mem::swap(&mut self.entities[index], &mut entity);
        id
    }

    pub fn despawn(&mut self, entity_id: Id) {
        let Some(index) = self
            .entities
            .iter()
            .position(|v| v.as_ref().is_some_and(|v| v.0 == entity_id))
        else {
            println!("tried to despawn {entity_id}; entity not found");
            return;
        };

        self.entities[index].take();
    }

    pub fn add_system<S, CTor>(&mut self, system_ctor: CTor) -> Id
    where
        S: System + 'static,
        CTor: Fn(Id) -> S,
    {
        let id = *self.system_id_counter;
        *self.system_id_counter += 1;
        let system = Rc::new(system_ctor(id));
        self.systems.push((id, system.clone()));
        system.on_add(self).unwrap();
        id
    }

    pub fn remove_system(&mut self, system_id: Id) {
        self.systems_to_remove.push(system_id);
    }

    pub fn key_just_pressed(&self, keycode: Keycode) -> bool {
        *self.currently_pressed_keys.get(&keycode).unwrap_or(&false)
    }

    pub fn key_pressed(&self, keycode: Keycode) -> bool {
        self.currently_pressed_keys.contains_key(&keycode)
    }

    pub fn mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        *self
            .currently_pressed_mouse_buttons
            .get(&button)
            .unwrap_or(&false)
    }

    pub fn mouse_position(&self) -> (i32, i32) {
        self.mouse_position
    }

    pub fn joystick_position(&self, id: Id) -> &ControllerPosition {
        &self
            .controllers
            .iter()
            .find(|v| v.0 == id)
            .map(|v| &v.2)
            .unwrap()
    }

    pub fn controller_button_pressed(&self, id: Id, button: ControllerButton) -> bool {
        self.currently_pressed_controller_buttons
            .contains_key(&(id, button))
    }

    pub fn controller_button_just_pressed(&self, id: Id, button: ControllerButton) -> bool {
        *self
            .currently_pressed_controller_buttons
            .get(&(id, button))
            .unwrap_or(&false)
    }

    pub fn active_controllers(&self) -> impl Iterator<Item = Id> {
        self.controllers
            .iter()
            .map(|v| v.0)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

pub struct DrawTextureOpts {
    pub color_mod: Option<(u8, u8, u8)>,
    pub opacity: Option<f64>,
    pub size: Option<V2>,
    pub angle: Option<f64>,
}

impl DrawTextureOpts {
    pub fn new() -> Self {
        Self {
            color_mod: None,
            opacity: None,
            size: None,
            angle: None,
        }
    }
    pub fn size(self, size: V2) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }
    pub fn color_mod(self, color_mod: (u8, u8, u8)) -> Self {
        Self {
            color_mod: Some(color_mod),
            ..self
        }
    }
    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            opacity: Some(opacity),
            ..self
        }
    }
    pub fn angle(self, angle: Option<f64>) -> Self {
        Self { angle, ..self }
    }
}
