#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use backend_connection::BackendConnection;
use engine::spawn;
use server::Server;

mod backend_connection;
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

fn main() {
    let mut backend_connection = BackendConnection::new();
    let mut server = Server::new(backend_connection.clone());

    let game_thread = std::thread::spawn(move || {
        let mut game = engine::Game::new().unwrap();

        let mut ctx = game.context();
        ctx.add_system(main_menu::MainMenuSystem);
        spawn!(&mut ctx, server.clone());

        game.run();
        server.quit();
    });

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        backend_connection.run().await;
    });

    game_thread.join().unwrap();
}
