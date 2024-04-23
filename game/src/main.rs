#![allow(dead_code)]

mod engine;
mod menu;
mod ui;

fn main() {
    let mut game = engine::Game::new().unwrap();

    let mut ctx = game.context();
    ctx.add_system(|id| ui::Menu0(id));

    game.run();
}
