#![allow(dead_code)]

use backend_connection::BackendConnection;
use engine::spawn;
use mock_connection::MockConnection;
use server::Server;

mod backend_connection;
mod game;
mod hero_creator;
mod hero_info;
mod hud;
mod hurtbox;
mod key_set;
mod knockoff;
mod main_menu;
mod mock_connection;
mod player;
mod player_attack;
mod player_movement;
mod server;
mod shared_ptr;
mod sprite_renderer;
mod start_game;
mod ui;

fn main() {
    // let mut connection = BackendConnection::new();
    let connection = MockConnection::new();
    let mut server = Server::new(connection.clone());

    let game_thread = std::thread::spawn(move || {
        let mut game = engine::Game::new().unwrap();

        let mut ctx = game.context();
        ctx.add_system(main_menu::MainMenuSystem);
        spawn!(&mut ctx, server.clone());

        game.run();
        server.quit();
    });

    // tokio::runtime::Runtime::new().unwrap().block_on(async {
    //     backend_connection.run().await;
    // });

    game_thread.join().unwrap();
}
