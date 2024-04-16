use std::{any::TypeId, collections::HashSet, rc::Rc};

use sdl2::{
    image::LoadTexture,
    keyboard::Keycode,
    mouse::MouseButton,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use super::{entity::Entity, id::Id, sprite::Sprite, system::System, Component, Error};

pub struct Context<'context, 'game>
where
    'game: 'context,
{
    pub(super) id_counter: &'context mut Id,
    pub(super) canvas: &'context mut Canvas<Window>,
    pub(super) texture_creator: *const TextureCreator<WindowContext>,
    pub(super) entities: &'context mut Vec<Entity>,
    pub(super) systems: &'context mut Vec<Rc<dyn System>>,
    pub(super) textures: &'context mut Vec<(Id, Texture<'game>)>,
    pub(super) currently_pressed_keys: &'context HashSet<Keycode>,
    pub(super) currently_pressed_mouse_buttons: &'context HashSet<MouseButton>,
    pub(super) mouse_position: (i32, i32),
}

pub struct ComponentQuery<T>(std::marker::PhantomData<T>);

impl<T> ComponentQuery<T> {
    pub fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

pub trait QueryRunner {
    fn run(&self, context: &Context) -> Vec<u64>;
}

impl<T0> QueryRunner for ComponentQuery<T0>
where
    T0: 'static + Component,
{
    fn run(&self, context: &Context) -> Vec<u64> {
        context.entities_with_component::<T0>()
    }
}

impl<T0, T1> QueryRunner for ComponentQuery<(T0, T1)>
where
    T0: 'static + Component,
    T1: 'static + Component,
{
    fn run(&self, context: &Context) -> Vec<u64> {
        let vs0 = context.entities_with_component::<T0>();
        let vs1 = context.entities_with_component::<T1>();
        vs0.into_iter()
            .filter(|v0| vs1.iter().any(|v1| *v0 == *v1))
            .collect()
    }
}

impl<T0, T1, T2> QueryRunner for ComponentQuery<(T0, T1, T2)>
where
    T0: 'static + Component,
    T1: 'static + Component,
    T2: 'static + Component,
{
    fn run(&self, context: &Context) -> Vec<u64> {
        let vs0 = context.entities_with_component::<T0>();
        let vs1 = context.entities_with_component::<T1>();
        let vs2 = context.entities_with_component::<T2>();
        vs0.into_iter()
            .filter(|v0| vs1.iter().any(|v1| *v0 == *v1) && vs2.iter().any(|v2| *v0 == *v2))
            .collect()
    }
}

#[macro_export]
macro_rules! query {
    ($ctx:expr, $t:ty) => {
        {
            #[allow(unused_imports)]
            use $crate::engine::QueryRunner;
            engine::ComponentQuery::<$t>::new().run($ctx)
        }
    };
    ($ctx:expr, $($ts:ty),+) => {
        {
            #[allow(unused_imports)]
            use $crate::engine::QueryRunner;
            engine::ComponentQuery::<($($ts),+)>::new().run($ctx)
        }
    };
}

#[macro_export]
macro_rules! spawn {
    ($ctx:expr, [$($ts:expr),+ $(,)?]) => {
        engine::Context::spawn($ctx, vec![$(Box::new($ts)),+])
    };
    ($ctx:expr, $($ts:expr),+ $(,)?) => {
        engine::Context::spawn($ctx, vec![$(Box::new($ts)),+])
    };
}

impl<'context, 'game> Context<'context, 'game> {
    pub fn entities_with_component<T: 'static + Component>(&self) -> Vec<u64> {
        let entity_type_id = TypeId::of::<T>();
        self.entities
            .iter()
            .filter_map(|Entity(id, components)| {
                let contains_component = components
                    .iter()
                    .any(|entity| (*entity).inner_type_id() == entity_type_id);
                if contains_component {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn entity_component<T: 'static + Component>(&mut self, entity_id: u64) -> &mut T {
        let entity_type_id = TypeId::of::<T>();
        let Entity(_id, components) = self
            .entities
            .iter_mut()
            .find(|Entity(id, _)| *id == entity_id)
            .unwrap();

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

    pub fn load_sprite<P>(&mut self, path: P) -> Result<Sprite, Error>
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref().to_path_buf();
        let texture: Texture<'game> = unsafe { (*self.texture_creator).load_texture(&path)? };
        let id = *self.id_counter;
        *self.id_counter += 1;
        self.textures.push((id, texture));
        Ok(Sprite(id))
    }

    pub fn draw_sprite(&mut self, sprite: Sprite, x: i32, y: i32) -> Result<(), Error> {
        let texture = self
            .textures
            .iter()
            .find_map(|v| if v.0 == sprite.0 { Some(&v.1) } else { None })
            .ok_or("invalid sprite id")?;
        self.canvas.copy(
            texture,
            None,
            Rect::new(x, y, texture.query().width, texture.query().height),
        )?;
        Ok(())
    }

    pub fn spawn(&mut self, components: Vec<Box<dyn Component>>) {
        let id = *self.id_counter;
        *self.id_counter += 1;
        self.entities.push(Entity(id, components));
    }

    pub fn despawn(&mut self, entity_id: u64) {
        *self.entities = self
            .entities
            .drain(..)
            .filter(|v| v.0 != entity_id)
            .collect();
    }

    pub fn add_system<S: 'static + System>(&mut self, system: S) {
        system.on_add(self);
        self.systems.push(Rc::new(system))
    }

    pub fn key_pressed(&self, keycode: Keycode) -> bool {
        self.currently_pressed_keys.contains(&keycode)
    }

    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.currently_pressed_mouse_buttons.contains(&button)
    }

    pub fn mouse_position(&self) -> (i32, i32) {
        self.mouse_position
    }
}
