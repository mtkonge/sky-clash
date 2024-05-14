use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use engine::Component;

#[derive(Clone, Debug)]
pub enum HeroResult {
    Hero(shared::Hero),
    UnknownRfid(String),
}

#[derive(Clone, Debug)]
pub struct Board {
    pub hero_1: Option<HeroResult>,
    pub hero_2: Option<HeroResult>,
}

pub trait ServerStrategy {
    fn quit(&mut self);
    fn update_hero_stats(&mut self, params: shared::UpdateHeroStatsParams);
    fn create_hero(&mut self, params: shared::CreateHeroParams);
    fn board_status(&mut self, res_pipe: Pipe<Board>);
}

// TODO this should pwobably only be in backend_connection
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

pub trait Res<T> {
    fn try_receive(&mut self) -> Option<T>;
}

#[derive(Component, Clone)]
pub struct Server {
    strategy: Arc<Mutex<dyn ServerStrategy + Send>>,
}

impl Server {
    pub fn new(strategy: impl ServerStrategy + Send + 'static) -> Self {
        Self {
            strategy: Arc::new(Mutex::new(strategy)),
        }
    }

    pub fn quit(&mut self) {
        self.strategy.lock().unwrap().quit()
    }

    pub fn update_hero_stats(&mut self, params: shared::UpdateHeroStatsParams) {
        self.strategy.lock().unwrap().update_hero_stats(params)
    }

    pub fn create_hero(&mut self, params: shared::CreateHeroParams) {
        self.strategy.lock().unwrap().create_hero(params)
    }

    pub fn board_status(&mut self, res_pipe: Pipe<Board>) {
        self.strategy.lock().unwrap().board_status(res_pipe)
    }
}
