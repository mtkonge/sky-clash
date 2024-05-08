use crate::actor::{self, Actor, Handle};
use core::panic;

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

type ResponseHandle = Handle<Result<HeroResult, String>>;

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

impl ServerActor {
    pub fn new(response_handle: Handle<Result<HeroResult, String>>) -> Self {
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
                    #[allow(unused_assignments)]
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
                            self.response_handle.send(Err(err));
                            break;
                        }
                    };

                    match reqwest::get(format!("http://65.108.91.32:8080/hero/{}", hero_rfid)).await
                    {
                        Ok(res) => {
                            let body = res.json::<Option<shared::Hero>>().await.unwrap();
                            let body = body
                                .map(HeroResult::Hero)
                                .unwrap_or(HeroResult::UnknownRfid(hero_rfid));

                            self.response_handle.send(Ok(body));
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
}
