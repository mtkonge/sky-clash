use std::sync::MutexGuard;

use engine::{query, spawn, Component, System};
use shared::Hero;

use crate::{
    game::GameSystem, hero_creator::change_image_node_content, hero_info::HeroInfo,
    server::HeroResult, shared_ptr::SharedPtr, ui, GameActor,
};

#[derive(Component, Clone)]
pub struct StartGame {
    system_id: u64,
    dom: SharedPtr<ui::Dom>,
}

fn handle_hero_result(
    hero: Option<HeroResult>,
    dom_id: u64,
    dom: &mut MutexGuard<ui::Dom>,
) -> Result<Hero, String> {
    match hero {
        Some(hero) => match hero {
            HeroResult::Hero(hero) => {
                change_image_node_content(
                    dom.select_mut(dom_id),
                    HeroInfo::from(&hero.hero_type).texture_path,
                );
                Ok(hero)
            }
            HeroResult::UnknownRfid(_) => {
                Err("uhhmm hero with rfid does not acshually exist :nerd:".to_string())
            }
        },
        None => Err("No hero found".to_string()),
    }
}

pub struct StartGameSystem(pub u64);
impl System for StartGameSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui::components::*;
        use ui::constructors::*;

        let system_id = self.0;

        let mut dom = ui::Dom::new(
            Stack([Hori([
                Vert([
                    Rect().with_height(300),
                    Image("./textures/placeholder.png")
                        .with_id(10u64)
                        .with_width(200)
                        .with_height(200)
                        .with_background_color((255, 0, 0)),
                ]),
                Rect().with_width(200),
                Vert([
                    Rect().with_height(400),
                    Button("Start Game")
                        .with_color((255, 255, 255))
                        .with_padding(15)
                        .on_click(0),
                ]),
                Rect().with_width(200),
                Vert([
                    Rect().with_height(300),
                    Rect()
                        .with_width(200)
                        .with_height(200)
                        .with_background_color((0, 255, 0)),
                ]),
            ])])
            .with_background_color((50, 50, 50))
            .with_width(1280)
            .with_height(720),
        );

        dom.add_event_handler(0, move |_dom, ctx, _node_id| {
            ctx.remove_system(system_id);
            ctx.add_system(GameSystem);
        });
        spawn!(
            ctx,
            StartGame {
                system_id: self.0,
                dom: SharedPtr::new(dom)
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        let start_game = ctx.clone_one::<StartGame>();
        start_game.dom.lock().update(ctx);

        let mut dom = start_game.dom.lock();

        let comms = ctx.select_one::<GameActor>();
        comms.server.send(crate::Message::BoardStatus);

        let heroes = comms.inner.try_receive();

        match heroes {
            Some(heroes) => {
                match handle_hero_result(heroes.hero_1, 10u64, &mut dom) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("{}", err);
                    }
                };
                match handle_hero_result(heroes.hero_2, 10u64, &mut dom) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("{}", err);
                    }
                };
            }
            None => return Ok(()),
        }
        Ok(())
    }

    fn on_remove(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        for id in query!(ctx, StartGame) {
            let start_game = ctx.select::<StartGame>(id).clone();
            if start_game.system_id == self.0 {
                ctx.despawn(id);
            }
        }

        Ok(())
    }
}
