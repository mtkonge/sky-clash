#![allow(dead_code)]

use comms::HeroOrRfid;
use engine::spawn;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::channel;

use crate::comms::{CommReq, Comms};

mod comms;
mod hero_creator;
mod hero_info;
mod main_menu;
mod ui;

#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    pub hero_1_rfid: Option<String>,
    pub hero_2_rfid: Option<String>,
}

fn main() {
    let (req_sender, req_receiver) = channel::<CommReq>();
    let (board_sender, board_receiver) = channel::<Result<HeroOrRfid, String>>();

    let game_thread = std::thread::spawn(move || {
        let mut game = engine::Game::new().unwrap();

        let mut ctx = game.context();
        ctx.add_system(main_menu::MainMenuSystem);
        spawn!(
            &mut ctx,
            Comms {
                req_sender: req_sender.clone(),
                board_receiver,
            }
        );

        game.run();
        req_sender.clone().send(CommReq::Quit).unwrap();
    });

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        comms::listen(req_receiver, board_sender).await;
    });

    game_thread.join().unwrap();
}
