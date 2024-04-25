#![allow(dead_code)]

mod engine;
mod my_menu;
mod ui;
mod ui2;

fn main() {
    let mut game = engine::Game::new().unwrap();

    let mut ctx = game.context();
    ctx.add_system(|id| ui::Menu0(id));

    game.run();
}
