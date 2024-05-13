use crate::actor::{self, Actor, Handle};
use core::panic;
use std::ops::ControlFlow;

use reqwest::header::HeaderMap;

#[derive(Clone, Debug)]
pub enum HeroResult {
    Hero(shared::Hero),
    UnknownRfid(String),
}

pub enum Message {
    Quit,
    BoardStatus,
    CreateHero(shared::CreateHeroParams),
    UpdateHeroStats(shared::UpdateHeroStatsParams),
}

pub struct BoardStateGoBrr {
    pub hero_1: Option<HeroResult>,
    pub hero_2: Option<HeroResult>,
}

type ResponseHandle = Handle<BoardStateGoBrr>;

pub struct ServerActor {
    inner: Actor<Message>,
    response_handle: ResponseHandle,
}

#[derive(Clone)]
pub struct Server(Handle<Message>);

impl std::ops::Deref for Server {
    type Target = Handle<Message>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Server {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

async fn hero_by_rfid(rfid: String) -> ControlFlow<(), HeroResult> {
    match reqwest::get(format!("http://65.108.91.32:8080/hero/{}", rfid)).await {
        Ok(res) => {
            let body = res.json::<Option<shared::Hero>>().await.unwrap();
            let body = body
                .map(HeroResult::Hero)
                .unwrap_or(HeroResult::UnknownRfid(rfid));
            ControlFlow::Continue(body)
        }
        Err(error) => {
            println!("e = {:?}", error);
            ControlFlow::Break(())
        }
    }
}

impl ServerActor {
    pub fn new(response_handle: ResponseHandle) -> Self {
        Self {
            inner: actor::Actor::new(),
            response_handle,
        }
    }
    pub fn handle(&self) -> Server {
        Server(self.inner.handle())
    }
    pub async fn run(mut self) {
        loop {
            let Some(message) = self.inner.try_receive() else {
                continue;
            };
            match message {
                Message::Quit => {
                    break;
                }

                Message::BoardStatus => {
                    #[allow(unused_variables)]
                    let board: shared::Board =
                        match reqwest::get("http://65.108.91.32:8080/heroes_on_board").await {
                            Ok(body) => body.json().await.unwrap(),
                            Err(error) => {
                                println!("e = {:?}", error);
                                break;
                            }
                        };
                    let board = shared::Board {
                        hero_1_rfid: Some("123452sda3".to_string()),
                        hero_2_rfid: Some("1234523".to_string()),
                    };

                    let hero_1 = match board.hero_1_rfid {
                        Some(rfid) => match hero_by_rfid(rfid).await {
                            ControlFlow::Continue(hero) => Some(hero),
                            ControlFlow::Break(()) => break,
                        },
                        None => None,
                    };
                    let hero_2 = match board.hero_2_rfid {
                        Some(rfid) => match hero_by_rfid(rfid).await {
                            ControlFlow::Continue(hero) => Some(hero),
                            ControlFlow::Break(()) => break,
                        },
                        None => None,
                    };

                    self.response_handle
                        .send(BoardStateGoBrr { hero_1, hero_2 })
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
}
