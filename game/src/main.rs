#![allow(dead_code)]

mod engine;
mod ui;

fn main() {
    let mut game = engine::Game::new().unwrap();

    let mut ctx = game.context();
    ctx.add_system(|_| ui::UI);

    game.run();
}
