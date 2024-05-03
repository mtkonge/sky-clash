use core::panic;
use std::sync::mpsc::{Receiver, Sender};

use engine::Component;
use reqwest::header::HeaderMap;
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

#[repr(i64)]
#[derive(serde_repr::Serialize_repr, Clone)]
pub enum HeroType {
    Centrist = 0,
    Strong = 1,
    Fast = 2,
    Tankie = 3,
}

#[derive(Serialize)]
pub struct CreateHeroParams {
    pub rfid: String,
    pub hero_type: HeroType,
}

pub enum CommReq {
    Quit,
    BoardStatus,
    CreateHero(CreateHeroParams),
}

pub async fn listen(
    req_receiver: Receiver<CommReq>,
    board_sender: Sender<Result<Option<Hero>, String>>,
) {
    loop {
        match req_receiver.recv().unwrap() {
            CommReq::Quit => {
                break;
            }

            CommReq::BoardStatus => {
                let mut board: Board =
                    match reqwest::get("http://sky.glowie.dk:8080/heroes_on_board").await {
                        Ok(body) => body.json().await.unwrap(),
                        Err(error) => {
                            println!("e = {:?}", error);
                            break;
                        }
                    };

                board = Board {
                    hero_1_rfid: Some("1234523".to_string()),
                    hero_2_rfid: None,
                };

                let hero_rfid = match (board.hero_1_rfid, board.hero_2_rfid) {
                    (None, Some(v)) | (Some(v), None) => Ok(v),
                    (None, None) => Err("please put 1 hero on board".to_string()),
                    (Some(_), Some(_)) => Err("please put only 1 hero on board".to_string()),
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
            CommReq::CreateHero(body) => {
                let client = reqwest::Client::new();
                let body_json = match serde_json::to_string(&body) {
                    Ok(body) => body,
                    Err(err) => {
                        panic!("Failed to serialize CreateHeroParams Err: {}", err)
                    }
                };
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                match client
                    .post("http://sky.glowie.dk:8080/create_hero")
                    .headers(headers)
                    .body(body_json)
                    .send()
                    .await
                {
                    Ok(v) => v,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };
            }
        }
    }
}
