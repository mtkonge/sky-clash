use std::collections::HashSet;
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

use super::{context::Context, entity::Entity, id::Id, system::System};
use super::{Component, Error};

pub struct Game<'a> {
    id_counter: u64,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    image_context: Sdl2ImageContext,
    ttf_context: Sdl2TtfContext,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    event_pump: sdl2::EventPump,
    entities: Vec<Entity>,
    components: Vec<(u64, Box<dyn Component>)>,
    systems: Vec<Rc<dyn System>>,
    textures: Vec<(Id, Texture<'a>)>,
    currently_pressed_keys: HashSet<Keycode>,
    currently_pressed_mouse_buttons: HashSet<MouseButton>,
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
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        let texture_creator = canvas.texture_creator();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        let event_pump = sdl_context.event_pump()?;
        let mouse_position = (event_pump.mouse_state().x(), event_pump.mouse_state().y());
        Ok(Self {
            id_counter: 0,
            sdl_context,
            video_subsystem,
            image_context,
            canvas,
            texture_creator,
            event_pump,
            ttf_context,
            entities: vec![],
            components: vec![],
            systems: vec![],
            textures: vec![],
            currently_pressed_keys: HashSet::new(),
            currently_pressed_mouse_buttons: HashSet::new(),
            mouse_position,
        })
    }

    pub fn run(&mut self) {
        let mut time_before = Instant::now();
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown { keycode, .. } => {
                        self.currently_pressed_keys.insert(keycode.unwrap());
                    }
                    Event::KeyUp { keycode, .. } => {
                        self.currently_pressed_keys.remove(&keycode.unwrap());
                    }
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        self.currently_pressed_mouse_buttons.insert(mouse_btn);
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
            for system in self.systems.clone() {
                let Err(err) = system.on_update(&mut self.context(), delta) else {
                    continue;
                };
                println!("error occcurred updating system: {err}");
            }
            self.canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144))
        }
    }

    pub fn context<'context>(&'context mut self) -> Context<'context, 'game>
    where
        'game: 'context,
    {
        Context {
            id_counter: &mut self.id_counter,
            canvas: &mut self.canvas,
            texture_creator: &self.texture_creator,
            ttf_context: &self.ttf_context,
            entities: &mut self.entities,
            systems: &mut self.systems,
            textures: &mut self.textures,
            currently_pressed_keys: &mut self.currently_pressed_keys,
            currently_pressed_mouse_buttons: &mut self.currently_pressed_mouse_buttons,
            mouse_position: self.mouse_position.clone(),
        }
    }

    pub fn add_system<S: 'static + System>(&mut self, system: S) {
        system.on_add(&mut self.context());
        self.systems.push(Rc::new(system))
    }
}
