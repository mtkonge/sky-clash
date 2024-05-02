use std::sync::mpsc::{Receiver, Sender};

use engine::Component;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    pub hero_1_rfid: Option<String>,
    pub hero_2_rfid: Option<String>,
}

#[derive(Component)]
pub struct Comms {
    req_sender: Sender<CommReq>,
    board_receiver: Receiver<Result<Option<Hero>, String>>,
}

pub enum CommReq {
    Quit,
    BoardStatus,
}
