use crate::server::{Board, HeroResult, Res, ServerStrategy};
use reqwest::header::HeaderMap;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

enum Message {
    Quit,
    BoardStatus(Pipe<Board>),
    CreateHero(shared::CreateHeroParams),
    UpdateHeroStats(shared::UpdateHeroStatsParams),
    UpdateBoardColors(shared::UpdateBoardColorsParams),
}

pub struct Pipe<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Pipe<T> {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn send(&mut self, value: T) {
        self.queue.lock().unwrap().push_back(value);
    }

    pub fn send_urgent(&mut self, value: T) {
        self.queue.lock().unwrap().push_front(value);
    }

    pub fn try_receive(&mut self) -> Option<T> {
        self.queue.lock().unwrap().pop_front()
    }
}

impl<T> Clone for Pipe<T> {
    fn clone(&self) -> Self {
        Self {
            queue: self.queue.clone(),
        }
    }
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
                                println!("e = {error:?}");
                                break;
                            }
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
                            panic!("Failed to serialize CreateHeroParams Err: {err}")
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
                            );
                        }
                        Err(err) => {
                            println!("{err}");
                            continue;
                        }
                    };
                }
                Message::UpdateHeroStats(body) => {
                    let client = reqwest::Client::new();
                    let body = match serde_json::to_string(&body) {
                        Ok(body) => body,
                        Err(err) => {
                            panic!("Failed to serialize UpdateHeroStatsParams Err: {err}")
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
                            let _ = response;
                        }
                        Err(err) => {
                            println!("{err}");
                            continue;
                        }
                    };
                }
                Message::UpdateBoardColors(body) => {
                    let client = reqwest::Client::new();
                    let body = match serde_json::to_string(&body) {
                        Ok(body) => body,
                        Err(err) => {
                            panic!("Failed to serialize UpdateBoardColorsParams Err: {err}")
                        }
                    };
                    let mut headers = HeaderMap::new();
                    headers.insert("Content-Type", "application/json".parse().unwrap());
                    match client
                        .post("http://65.108.91.32:8080/update_board_colors")
                        .headers(headers)
                        .body(body)
                        .send()
                        .await
                    {
                        Ok(response) => {
                            println!(
                                "update_board_colors response: {} '{}'",
                                response.status().as_str(),
                                response.text().await.unwrap()
                            );
                        }
                        Err(err) => {
                            println!("{err}");
                            continue;
                        }
                    };
                }
            }
        }
    }
}

#[derive(Clone)]
struct BoardStatusRes {
    res_pipe: Pipe<Board>,
}

impl Res<Board> for BoardStatusRes {
    fn try_receive(&mut self) -> Option<Board> {
        self.res_pipe.try_receive()
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

    fn board_status(&mut self) -> Box<dyn Res<Board>> {
        let res_pipe = Pipe::new();
        self.pipe.send(Message::BoardStatus(res_pipe.clone()));
        Box::new(BoardStatusRes { res_pipe })
    }

    fn update_board_colors(&mut self, params: shared::UpdateBoardColorsParams) {
        self.pipe.send(Message::UpdateBoardColors(params))
    }
}

async fn hero_by_rfid(rfid: String) -> Option<HeroResult> {
    match reqwest::get(format!("http://65.108.91.32:8080/hero/{rfid}")).await {
        Ok(res) => {
            let body = res.json::<Option<shared::Hero>>().await.unwrap();
            let body = body
                .map(HeroResult::Hero)
                .unwrap_or(HeroResult::UnknownRfid(rfid));
            Some(body)
        }
        Err(error) => {
            println!("e = {error:?}");
            None
        }
    }
}
