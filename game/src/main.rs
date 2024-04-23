#![allow(dead_code)]

mod engine;

fn main() {
    let mut game = engine::Game::new().unwrap();

    let mut ctx = game.context();

    game.run();
}
