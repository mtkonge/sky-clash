#![allow(dead_code)]

use hero_creator::HeroCreator;
use main_menu::MainMenu;

mod engine;
mod hero_creator;
mod main_menu;
mod ui;

fn main() {
    let mut game = engine::Game::new().unwrap();

    let mut ctx = game.context();
    ctx.add_system(MainMenu);

    game.run();
}
