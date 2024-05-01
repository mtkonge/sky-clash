#![allow(dead_code)]

use engine::{spawn, Component};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{channel, Receiver, Sender};

mod hero_creator;
mod main_menu;
mod my_menu;
mod ui2;

#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    pub hero_1_rfid: Option<String>,
    pub hero_2_rfid: Option<String>,
}

#[derive(Component)]
pub struct Comms {
    i_want_board_top: Sender<()>,
    board_bottom: Receiver<Board>,
}

fn main() {
    let (sender, receiver) = channel::<String>();
    let (i_want_board_top, i_want_board_bottom) = channel::<()>();
    let (board_top, board_bottom) = channel::<Board>();

    let game_thread = std::thread::spawn(move || {
        let mut game = engine::Game::new().unwrap();

        let mut ctx = game.context();
        ctx.add_system(main_menu::MainMenuSystem);
        spawn!(
            &mut ctx,
            Comms {
                i_want_board_top,
                board_bottom,
            }
        );

        game.run();
    });

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        sender.send("hello".to_string()).unwrap();

        loop {
            let _ = i_want_board_bottom.recv();
            let board: Board = match reqwest::get("http://sky.glowie.dk:8080/heroes_on_board").await
            {
                Ok(body) => body.json().await.unwrap(),
                Err(error) => {
                    println!("e = {:?}", error);
                    break;
                }
            };
            board_top.send(board).unwrap();
        }
    });

    println!("{}", receiver.recv().unwrap());

    game_thread.join().unwrap();
}
