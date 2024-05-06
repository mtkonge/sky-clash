use core::panic;
use std::sync::mpsc::{Receiver, Sender};

use engine::Component;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::hero_info::{HeroStats, HeroType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hero {
    pub id: i64,
    pub rfid: String,
    pub level: i64,
    pub hero_type: HeroType,
    pub unallocated_skillpoints: i64,
    pub strength_points: i64,
    pub agility_points: i64,
    pub defence_points: i64,
}

#[derive(Clone, Debug)]
pub enum HeroOrUnknownRfid {
    Hero(Hero),
    Rfid(String),
}

#[derive(Component)]
pub struct Comms {
    pub req_sender: Sender<Message>,
    pub board_receiver: Receiver<Result<HeroOrUnknownRfid, String>>,
}

#[derive(Serialize)]
pub struct CreateHeroParams {
    pub rfid: String,
    pub hero_type: HeroType,
    pub base_stats: HeroStats,
}

#[derive(Serialize)]
pub struct UpdateHeroStatsParams {
    pub rfid: String,
    pub stats: HeroStats,
}

pub enum Message {
    Quit,
    BoardStatus,
    CreateHero(CreateHeroParams),
    UpdateHeroStats(UpdateHeroStatsParams),
}

pub async fn listen(
    req_receiver: Receiver<Message>,
    board_sender: Sender<Result<HeroOrUnknownRfid, String>>,
) {
    loop {
        match req_receiver.recv().unwrap() {
            Message::Quit => {
                break;
            }

            Message::BoardStatus => {
                let mut board: shared::Board =
                    match reqwest::get("http://65.108.91.32:8080/heroes_on_board").await {
                        Ok(body) => body.json().await.unwrap(),
                        Err(error) => {
                            println!("e = {:?}", error);
                            break;
                        }
                    };

                board = shared::Board {
                    hero_1_rfid: Some("1234523".to_string()),
                    hero_2_rfid: None,
                };

                let hero_rfid = match (board.hero_1_rfid, board.hero_2_rfid) {
                    (None, Some(v)) | (Some(v), None) => Ok(v),
                    (None, None) => Err("please put 1 hero on board".to_string()),
                    (Some(_), Some(_)) => Err("please put only 1 hero on board".to_string()),
                };
                let hero_rfid = match hero_rfid {
                    Ok(rfid) => rfid,
                    Err(err) => {
                        board_sender.send(Err(err)).unwrap();
                        break;
                    }
                };

                match reqwest::get(format!("http://65.108.91.32:8080/hero/{}", hero_rfid)).await {
                    Ok(res) => {
                        let body = res.json::<Option<Hero>>().await.unwrap();
                        let body = body
                            .map(HeroOrUnknownRfid::Hero)
                            .unwrap_or(HeroOrUnknownRfid::Rfid(hero_rfid));

                        board_sender.send(Ok(body)).unwrap();
                    }
                    Err(error) => {
                        println!("e = {:?}", error);
                        break;
                    }
                };
            }
            Message::CreateHero(body) => {
                let client = reqwest::Client::new();
                let body = match serde_json::to_string(&body) {
                    Ok(body) => body,
                    Err(err) => {
                        panic!("Failed to serialize CreateHeroParams Err: {}", err)
                    }
                };
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                match client
                    .post("http://65.108.91.32:8080/create_hero")
                    .headers(headers)
                    .body(body)
                    .send()
                    .await
                {
                    Ok(response) => {
                        println!(
                            "create_hero response: {} '{}'",
                            response.status().as_str(),
                            response.text().await.unwrap()
                        )
                    }
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };
            }
            Message::UpdateHeroStats(body) => {
                let client = reqwest::Client::new();
                let body = match serde_json::to_string(&body) {
                    Ok(body) => body,
                    Err(err) => {
                        panic!("Failed to serialize UpdateHeroStatsParams Err: {}", err)
                    }
                };
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                match client
                    .post("http://65.108.91.32:8080/update_hero_stats")
                    .headers(headers)
                    .body(body)
                    .send()
                    .await
                {
                    Ok(response) => {
                        println!(
                            "update_hero_stats response: {} '{}'",
                            response.status().as_str(),
                            response.text().await.unwrap()
                        )
                    }
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };
            }
        }
    }
}
