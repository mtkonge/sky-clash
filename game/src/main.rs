#![allow(dead_code)]

mod my_menu;
mod ui2;

fn main() {
    let mut game = engine::Game::new().unwrap();

    let mut ctx = game.context();
    ctx.add_system(my_menu::MyMenuSystem);

    game.run();
}
