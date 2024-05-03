#![allow(dead_code)]

use comms::{Hero, HeroType};
use engine::{spawn, Component};
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::comms::CommReq;

mod comms;
mod hero_creator;
mod main_menu;
mod ui;

#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    pub hero_1_rfid: Option<String>,
    pub hero_2_rfid: Option<String>,
}

pub struct HeroStats {
    strength: u8,
    agility: u8,
    defence: u8,
}

pub struct HeroInfo {
    pub base_stats: HeroStats,
    pub texture_path: PathBuf,
}

impl From<HeroType> for HeroInfo {
    fn from(value: HeroType) -> Self {
        match value {
            HeroType::Centrist => HeroInfo {
                base_stats: HeroStats {
                    strength: 8,
                    agility: 8,
                    defence: 8,
                },
                texture_path: PathBuf::from("./textures/sprites/grill.png"),
            },
            HeroType::Strong => HeroInfo {
                base_stats: HeroStats {
                    strength: 12,
                    agility: 8,
                    defence: 4,
                },
                texture_path: PathBuf::from("./textures/sprites/strong.png"),
            },
            HeroType::Fast => HeroInfo {
                base_stats: HeroStats {
                    strength: 4,
                    agility: 12,
                    defence: 8,
                },
                texture_path: PathBuf::from("./textures/sprites/speed.png"),
            },
            HeroType::Tankie => HeroInfo {
                base_stats: HeroStats {
                    strength: 8,
                    agility: 4,
                    defence: 12,
                },
                texture_path: PathBuf::from("./textures/sprites/tankie.png"),
            },
        }
    }
}

#[derive(Component)]
pub struct Comms {
    req_sender: Sender<CommReq>,
    board_receiver: Receiver<Result<Option<Hero>, String>>,
}

fn main() {
    let (req_sender, req_receiver) = channel::<CommReq>();
    let (board_sender, board_receiver) = channel::<Result<Option<Hero>, String>>();

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
