use std::sync::{Arc, Mutex};

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

pub trait Res<T> {
    fn try_receive(&mut self) -> Option<T>;
}

pub trait ServerStrategy {
    fn quit(&mut self);
    fn update_hero_stats(&mut self, params: shared::UpdateHeroStatsParams);
    fn create_hero(&mut self, params: shared::CreateHeroParams);
    fn board_status(&mut self) -> Box<dyn Res<Board>>;
    fn update_board_colors(&mut self, params: shared::UpdateBoardColorsParams);
    fn create_match(&mut self, params: shared::CreateMatchParams);
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
        self.strategy.lock().unwrap().quit();
    }

    pub fn update_hero_stats(&mut self, params: shared::UpdateHeroStatsParams) {
        self.strategy.lock().unwrap().update_hero_stats(params);
    }

    pub fn create_hero(&mut self, params: shared::CreateHeroParams) {
        self.strategy.lock().unwrap().create_hero(params);
    }

    pub fn board_status(&mut self) -> Box<dyn Res<Board>> {
        self.strategy.lock().unwrap().board_status()
    }

    pub fn update_board_colors(&mut self, params: shared::UpdateBoardColorsParams) {
        self.strategy.lock().unwrap().update_board_colors(params)
    }

    pub fn create_match(&mut self, params: shared::CreateMatchParams) {
        self.strategy.lock().unwrap().create_match(params)
    }
}
