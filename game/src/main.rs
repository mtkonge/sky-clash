#![allow(dead_code)]

use std::sync::mpsc::channel;

mod main_menu;
mod my_menu;
mod ui2;

fn main() {
    let (sender, receiver) = channel::<String>();

    let game_thread = std::thread::spawn(|| {
        let mut game = engine::Game::new().unwrap();

        let mut ctx = game.context();
        ctx.add_system(main_menu::MainMenuSystem);

        game.run();
    });

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            sender.send("hello".to_string()).unwrap();
        });

    println!("{}", receiver.recv().unwrap());

    game_thread.join().unwrap();
}
