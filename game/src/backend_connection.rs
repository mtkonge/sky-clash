use reqwest::header::HeaderMap;

use crate::server::{Board, HeroResult, Pipe, Res, ServerStrategy};

enum Message {
    Quit,
    BoardStatus(Pipe<Board>),
    CreateHero(shared::CreateHeroParams),
    UpdateHeroStats(shared::UpdateHeroStatsParams),
}

#[derive(Clone)]
pub struct BackendConnection {
    pipe: Pipe<Message>,
}

impl BackendConnection {
    pub fn new() -> Self {
        Self { pipe: Pipe::new() }
    }
}

impl BackendConnection {
    pub async fn run(&mut self) {
        loop {
            let Some(message) = self.pipe.try_receive() else {
                continue;
            };
            match message {
                Message::Quit => break,
                Message::BoardStatus(mut res_pipe) => {
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
                        Some(rfid) => hero_by_rfid(rfid).await,
                        None => None,
                    };
                    let hero_2 = match board.hero_2_rfid {
                        Some(rfid) => hero_by_rfid(rfid).await,
                        None => None,
                    };

                    res_pipe.send(Board { hero_1, hero_2 });
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

impl ServerStrategy for BackendConnection {
    fn quit(&mut self) {
        self.pipe.send_urgent(Message::Quit);
    }

    fn update_hero_stats(&mut self, params: shared::UpdateHeroStatsParams) {
        self.pipe.send_urgent(Message::UpdateHeroStats(params));
    }

    fn create_hero(&mut self, params: shared::CreateHeroParams) {
        self.pipe.send_urgent(Message::CreateHero(params));
    }

    fn board_status(&mut self, res_pipe: Pipe<Board>) {
        self.pipe.send(Message::BoardStatus(res_pipe));
    }
}

async fn hero_by_rfid(rfid: String) -> Option<HeroResult> {
    match reqwest::get(format!("http://65.108.91.32:8080/hero/{}", rfid)).await {
        Ok(res) => {
            let body = res.json::<Option<shared::Hero>>().await.unwrap();
            let body = body
                .map(HeroResult::Hero)
                .unwrap_or(HeroResult::UnknownRfid(rfid));
            Some(body)
        }
        Err(error) => {
            println!("e = {:?}", error);
            None
        }
    }
}
