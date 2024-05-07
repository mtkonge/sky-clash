#![allow(dead_code)]

use crate::message::{HeroResult, Message, MothershipActor};
use actor::Actor;
use engine::{spawn, Component};
use message::MothershipHandle;

mod actor;
mod game;
mod hero_creator;
mod hero_info;
mod main_menu;
mod message;
mod player_movement;
mod shared_ptr;
mod sprite_renderer;
mod start_game;
mod ui;

#[derive(Component)]
pub struct GameActor {
    inner: Actor<Result<HeroResult, String>>,
    mothership_handle: MothershipHandle,
}

fn main() {
    let game_actor = Actor::new();
    let mothership_actor = MothershipActor::new(game_actor.handle());
    let game_actor = GameActor {
        inner: game_actor,
        mothership_handle: mothership_actor.handle(),
    };

    let game_thread = std::thread::spawn(move || {
        let mut game = engine::Game::new().unwrap();

        let mut ctx = game.context();
        ctx.add_system(main_menu::MainMenuSystem);
        let mut quit_handle = game_actor.mothership_handle.clone();
        spawn!(&mut ctx, game_actor);

        game.run();
        quit_handle.send(Message::Quit);
    });

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        mothership_actor.run().await;
    });

    game_thread.join().unwrap();
}
