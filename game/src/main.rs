#![allow(dead_code)]

use engine::spawn;
use server::Server;

mod attacks;
mod backend_connection;
mod game;
mod hero_creator;
mod hero_info;
mod hud;
mod hurtbox;
mod keyset;
mod knockoff;
mod main_menu;
mod mock_connection;
mod player;
mod player_interaction;
mod server;
mod sprite_renderer;
mod start_game;
mod timer;

// pub const FONT: &str = "textures/ttf/OpenSans.ttf";
pub const FONT: &str = "textures/ttf/Jaro-Regular.ttf";

fn main() {
    // let mut connection = backend_connection::BackendConnection::new();
    let connection = mock_connection::MockConnection::new();
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
    //     connection.run().await;
    // });

    game_thread.join().unwrap();
}
