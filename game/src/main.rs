#![allow(dead_code)]

use crate::server::{Message, ServerActor};
use actor::Actor;
use engine::{spawn, Component};
use server::Server;

mod actor;
mod game;
mod hero_creator;
mod hero_info;
mod main_menu;
mod player_movement;
mod server;
mod shared_ptr;
mod sprite_renderer;
mod start_game;
mod ui;

#[derive(Component)]
pub struct GameActor {
    inner: Actor<server::BoardStateGoBrr>,
    server: Server,
}

fn main() {
    let game_actor = Actor::new();
    let server_actor = ServerActor::new(game_actor.handle());
    let game_actor = GameActor {
        inner: game_actor,
        server: server_actor.handle(),
    };

    let game_thread = std::thread::spawn(move || {
        let mut game = engine::Game::new().unwrap();

        let mut ctx = game.context();
        ctx.add_system(main_menu::MainMenuSystem);
        let mut quit_handle = game_actor.server.clone();
        spawn!(&mut ctx, game_actor);

        game.run();
        quit_handle.send_important(Message::Quit);
    });

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        server_actor.run().await;
    });

    game_thread.join().unwrap();
}
