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

#[derive(Serialize, Deserialize, Clone)]
pub struct Hero {
    pub id: i64,
    pub rfid: String,
    pub level: i64,
    pub hero_type: i64,
    pub unallocated_skillpoints: i64,
    pub strength_points: i64,
    pub agility_points: i64,
    pub defence_points: i64,
}

#[derive(Component)]
pub struct Comms {
    i_want_board_sender: Sender<()>,
    board_receiver: Receiver<Result<Option<Hero>, String>>,
}

fn main() {
    let (sender, receiver) = channel::<String>();
    let (i_want_board_sender, i_want_board_receiver) = channel::<()>();
    let (board_sender, board_receiver) = channel::<Result<Option<Hero>, String>>();

    let game_thread = std::thread::spawn(move || {
        let mut game = engine::Game::new().unwrap();

        let mut ctx = game.context();
        ctx.add_system(main_menu::MainMenuSystem);
        spawn!(
            &mut ctx,
            Comms {
                i_want_board_sender,
                board_receiver,
            }
        );

        game.run();
    });

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        sender.send("hello".to_string()).unwrap();

        loop {
            let _ = i_want_board_receiver.recv();
            let board: Board = match reqwest::get("http://sky.glowie.dk:8080/heroes_on_board").await
            {
                Ok(body) => body.json().await.unwrap(),
                Err(error) => {
                    println!("e = {:?}", error);
                    break;
                }
            };

            let hero_rfid = match (board.hero_1_rfid, board.hero_2_rfid) {
                (None, Some(v)) | (Some(v), None) => Ok(v),
                (None, None) => Err("atleast 1 hero on board plz".to_string()),
                (Some(_), Some(_)) => Err("max 1 hero on board plz".to_string()),
            };
            let Ok(hero_rfid) = hero_rfid else {
                board_sender.send(hero_rfid.map(|_| None)).unwrap();
                break;
            };

            match reqwest::get(format!("http://sky.glowie.dk:8080/hero/{}", hero_rfid)).await {
                Ok(res) => board_sender
                    .send(Ok(res.json::<Option<Hero>>().await.unwrap()))
                    .unwrap(),
                Err(error) => {
                    println!("e = {:?}", error);
                    break;
                }
            };
        }
    });

    println!("{}", receiver.recv().unwrap());

    game_thread.join().unwrap();
}
