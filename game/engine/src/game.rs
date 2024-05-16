use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::ttf::{self, Sdl2TtfContext};
use sdl2::{
    event::Event,
    image::{self, Sdl2ImageContext},
    pixels::Color,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    Sdl, VideoSubsystem,
};

use crate::texture::TextTextureKey;
use crate::Text;

use super::font::Font;
use super::{context::Context, entity::Entity, id::Id, system::System};
use super::{Component, Error};

pub struct Game<'game> {
    #[allow(unused)]
    sdl_context: Sdl,
    #[allow(unused)]
    video_subsystem: VideoSubsystem,
    #[allow(unused)]
    image_context: Sdl2ImageContext,
    ttf_context: Sdl2TtfContext,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    event_pump: sdl2::EventPump,
    entity_id_counter: Id,
    entities: Vec<Option<Entity>>,
    system_id_counter: Id,
    systems: Vec<(Id, Rc<dyn System>)>,
    systems_to_remove: Vec<Id>,
    textures: Vec<(Id, Texture<'game>)>,
    texture_path_to_id_map: HashMap<PathBuf, Id>,
    text_textures: HashMap<TextTextureKey, Text>,
    fonts: Vec<(Id, u16, PathBuf, Font<'game>)>,
    currently_pressed_keys: HashMap<Keycode, bool>,
    currently_pressed_mouse_buttons: HashMap<MouseButton, bool>,
    mouse_position: (i32, i32),
}

impl<'game> Game<'game> {
    pub fn new() -> Result<Self, Error> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let image_context = image::init(image::InitFlag::PNG)?;
        let ttf_context = ttf::init().map_err(|e| e.to_string())?;
        let window = video_subsystem
            .window("pvp-game-dilapidation", 1280, 720)
            .position_centered()
            // .fullscreen()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        let texture_creator = canvas.texture_creator();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        let event_pump = sdl_context.event_pump()?;
        let mouse_position = (event_pump.mouse_state().x(), event_pump.mouse_state().y());
        Ok(Self {
            sdl_context,
            video_subsystem,
            image_context,
            canvas,
            texture_creator,
            event_pump,
            ttf_context,
            entity_id_counter: 0,
            entities: vec![],
            system_id_counter: 0,
            systems: vec![],
            systems_to_remove: vec![],
            textures: vec![],
            texture_path_to_id_map: HashMap::new(),
            text_textures: HashMap::new(),
            fonts: vec![],
            currently_pressed_keys: HashMap::new(),
            currently_pressed_mouse_buttons: HashMap::new(),
            mouse_position,
        })
    }

    pub fn run(&mut self) {
        let mut time_before = Instant::now();
        let time_per_frame = 1_000_000_000 / 144;
        'running: loop {
            self.currently_pressed_mouse_buttons
                .values_mut()
                .for_each(|value| {
                    *value = false;
                });
            self.currently_pressed_keys.values_mut().for_each(|value| {
                *value = false;
            });
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown { keycode, .. } => {
                        self.currently_pressed_keys.insert(keycode.unwrap(), true);
                    }
                    Event::KeyUp { keycode, .. } => {
                        self.currently_pressed_keys.remove(&keycode.unwrap());
                    }
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        self.currently_pressed_mouse_buttons.insert(mouse_btn, true);
                    }
                    Event::MouseButtonUp { mouse_btn, .. } => {
                        self.currently_pressed_mouse_buttons.remove(&mouse_btn);
                    }
                    _ => {}
                }
            }
            self.mouse_position = (
                self.event_pump.mouse_state().x(),
                self.event_pump.mouse_state().y(),
            );
            self.canvas.set_draw_color(Color::RGB(60, 180, 180));
            self.canvas.clear();
            let now = Instant::now();
            let delta = (now - time_before).as_nanos() as f64 / 1_000_000_000.0;
            time_before = now;

            for (_id, system) in self.systems.clone() {
                let Err(err) = system.on_update(&mut self.context(), delta) else {
                    continue;
                };
                println!("error occurred updating system: {err}");
            }
            let ids_to_remove: Vec<_> = self.systems_to_remove.drain(..).collect();
            for removed_id in ids_to_remove {
                let Some(position) = self.systems.iter().position(|(id, _)| *id == removed_id)
                else {
                    println!("tried to remove system with id {removed_id} but unable to");
                    continue;
                };
                let (_, system) = self.systems.remove(position);
                if let Err(err) = system.on_remove(&mut self.context()) {
                    println!("error occurred removing system: {err}");
                };
            }
            self.canvas.present();
            let update_duration = Instant::now() - now;
            let update_duration = update_duration.as_nanos();
            if time_per_frame > update_duration {
                std::thread::sleep(Duration::new(0, (time_per_frame - update_duration) as u32))
            }
        }
    }

    pub fn context<'context>(&'context mut self) -> Context<'context, 'game>
    where
        'game: 'context,
    {
        Context {
            canvas: &mut self.canvas,
            texture_creator: &self.texture_creator,
            ttf_context: &self.ttf_context,
            entity_id_counter: &mut self.entity_id_counter,
            entities: &mut self.entities,
            system_id_counter: &mut self.system_id_counter,
            systems: &mut self.systems,
            systems_to_remove: &mut self.systems_to_remove,
            textures: &mut self.textures,
            texture_path_to_id_map: &mut self.texture_path_to_id_map,
            text_textures: &mut self.text_textures,
            fonts: &mut self.fonts,
            currently_pressed_keys: &mut self.currently_pressed_keys,
            currently_pressed_mouse_buttons: &mut self.currently_pressed_mouse_buttons,
            mouse_position: self.mouse_position,
        }
    }
}
