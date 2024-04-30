use std::rc::Rc;
use std::sync::Mutex;

use crate::hero_creator::HeroCreatorSystem;
use crate::ui2;
use engine::{query, spawn};
use engine::{Component, System};

#[derive(Component, Clone)]
pub struct MainMenu {
    system_id: u64,
    dom: Rc<Mutex<ui2::Dom>>,
}

pub struct MainMenuSystem(pub u64);
impl System for MainMenuSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui2::constructors::*;
        let system_id = self.0;

        let mut dom = ui2::Dom::new(
            Vert([
                Rect().with_height(300),
                Text("SkyTrash").with_font_size(48),
                Text("Start Game")
                    .with_color((255, 255, 255))
                    .with_padding(15)
                    .with_border_thickness(2)
                    .with_border_color((0, 0, 0))
                    .on_click(1),
                Text("Hero Creator")
                    .with_color((255, 255, 255))
                    .with_padding(15)
                    .with_border_thickness(2)
                    .with_border_color((0, 0, 0))
                    .on_click(2),
                Text("Exit")
                    .with_color((255, 255, 255))
                    .with_padding(15)
                    .with_border_thickness(2)
                    .with_border_color((0, 0, 0))
                    .on_click(3),
            ])
            .with_width(1280)
            .with_height(720)
            .with_background_color((100, 100, 100)),
        );

        dom.add_event_handler(1, |_dom, _ctx, _node_id| {
            // let Some(element) = dom.select_mut(34) else {
            //     return;
            // };
            // if let ui2::Kind::Text { text, .. } = &mut element.kind {
            //     *text = "some thing else".to_string();
            // };
            println!("button clicked");
        });

        dom.add_event_handler(2, move |_dom, ctx, _node_id| {
            ctx.remove_system(system_id);
            ctx.add_system(HeroCreatorSystem);
        });

        dom.add_event_handler(3, |dom, _ctx, _node_id| {
            panic!("exit");
        });

        spawn!(
            ctx,
            MainMenu {
                system_id: self.0,
                dom: Rc::new(Mutex::new(dom))
            }
        );

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, MainMenu) {
            let main_menu = ctx.entity_component::<MainMenu>(id).clone();
            main_menu.dom.lock().unwrap().update(ctx);
        }
        Ok(())
    }

    fn on_remove(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        for id in query!(ctx, MainMenu) {
            let main_menu = ctx.entity_component::<MainMenu>(id).clone();
            if main_menu.system_id == self.0 {
                ctx.despawn(id);
            }
        }
        Ok(())
    }
}
