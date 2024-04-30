use std::rc::Rc;
use std::sync::Mutex;

use crate::ui2::{self, BoxedNode, Dom, Node, NodeId};
use crate::Comms;
use engine::{query, spawn};
use engine::{Component, System};

#[derive(Component, Clone)]
pub struct HeroCreator {
    dom: Rc<Mutex<ui2::Dom>>,
}

#[derive(Component, Default)]
struct HeroState {
    agility: usize,
    defence: usize,
    strength: usize,
}

#[allow(non_snake_case)]
fn Percentage<S: Into<String>>(
    text: S,
    filled_steps: usize,
    max_steps: usize,
    id_offset: u64,
) -> BoxedNode {
    use ui2::constructors::*;

    let middle = max_steps / 2;
    let mut children: Vec<_> = (0..max_steps)
        .map(|i| {
            if i < filled_steps {
                Text("|")
                    .with_color((255, 255, 255))
                    .with_id(i as u64 + id_offset + 1)
            } else {
                Text("|")
                    .with_color((127, 127, 127))
                    .with_id(i as u64 + id_offset + 1)
            }
        })
        .collect();
    children.insert(middle, Text(text).with_id(id_offset));
    children.insert(middle + 1, Text(" "));
    children.insert(middle, Text(" "));
    Hori(children)
        .with_padding(4)
        .with_border_color((255, 255, 255))
}

fn mutate_percentage_bar<S: Into<String>>(
    dom: &mut Dom,
    new_text: S,
    id_offset: u64,
    filled_steps: usize,
    max_steps: usize,
) {
    let Some(node) = dom.select_mut(id_offset) else {
        panic!("percentage does not exist");
    };

    match &mut node.kind {
        ui2::Kind::Text { text, .. } => {
            *text = new_text.into();
        }
        _ => panic!("item is not a percentage"),
    }

    for i in 0..max_steps {
        let Some(node) = dom.select_mut(i as u64 + id_offset + 1) else {
            panic!("percentage does not exist");
        };
        if i < filled_steps {
            node.set_color((255, 255, 255));
        } else {
            node.set_color((127, 127, 127));
        }
    }
}

const MAX_STEPS: usize = 24;

pub struct HeroCreatorSystem(pub u64);
impl System for HeroCreatorSystem {
    fn on_add(&self, ctx: &mut engine::Context) -> Result<(), engine::Error> {
        use ui2::constructors::*;

        let mut dom = ui2::Dom::new(
            Hori([Vert([
                Text("Retrieving board").with_id(0),
                Hori([
                    Text("-").on_click(10).with_padding(8),
                    Text(" "),
                    Percentage("Strength (00/24)", 0, 24, 100),
                    Text(" "),
                    Text("+").on_click(11).with_padding(8),
                ]),
                Hori([
                    Text("-").on_click(20).with_padding(8),
                    Text(" "),
                    Percentage("Agility (00/24)", 0, 24, 200),
                    Text(" "),
                    Text("+").on_click(21).with_padding(8),
                ]),
                Hori([
                    Text("-").on_click(30).with_padding(8),
                    Text(" "),
                    Percentage("Defence  (00/24)", 0, 24, 300),
                    Text(" "),
                    Text("+").on_click(31).with_padding(8),
                ]),
            ])
            .with_width(1280)])
            .with_width(1280)
            .with_height(720)
            .with_background_color((0, 0, 0)),
        );

        macro_rules! impl_da_ting {
            ($dom: ident, $id_offset: expr, $property: ident, $property_name: expr, $percentage_offset: expr) => {
                $dom.add_event_handler(
                    $id_offset,
                    |dom: &mut Dom, ctx: &mut engine::Context, _id: NodeId| {
                        for id in query!(ctx, HeroState) {
                            let state = ctx.entity_component::<HeroState>(id);
                            if state.$property == 0 {
                                return;
                            }
                            state.$property -= 1;
                            let new_text = format!(
                                "{} ({:02}/{MAX_STEPS:02})",
                                $property_name, state.$property
                            );
                            mutate_percentage_bar(
                                dom,
                                new_text,
                                $percentage_offset,
                                state.$property,
                                MAX_STEPS,
                            );
                        }
                    },
                );

                $dom.add_event_handler(
                    $id_offset + 1,
                    |dom: &mut Dom, ctx: &mut engine::Context, _id: NodeId| {
                        for id in query!(ctx, HeroState) {
                            let state = ctx.entity_component::<HeroState>(id);
                            if state.$property == MAX_STEPS {
                                return;
                            }
                            state.$property += 1;
                            let new_text = format!(
                                "{} ({:02}/{MAX_STEPS:02})",
                                $property_name, state.$property
                            );
                            mutate_percentage_bar(
                                dom,
                                new_text,
                                $percentage_offset,
                                state.$property,
                                MAX_STEPS,
                            );
                        }
                    },
                );
            };
        }

        for id in query!(ctx, Comms) {
            let comms = ctx.entity_component::<Comms>(id);
            comms.i_want_board_top.send(()).unwrap();
        }

        impl_da_ting!(dom, 10, strength, "Strength", 100);
        impl_da_ting!(dom, 20, agility, "Agility", 200);
        impl_da_ting!(dom, 30, defence, "Defence", 300);

        spawn!(
            ctx,
            HeroCreator {
                dom: Rc::new(Mutex::new(dom)),
            }
        );

        spawn!(ctx, HeroState::default());

        Ok(())
    }

    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, HeroCreator) {
            let menu = ctx.entity_component::<HeroCreator>(id).clone();
            menu.dom.lock().unwrap().update(ctx);

            for id in query!(ctx, Comms) {
                let comms = ctx.entity_component::<Comms>(id);
                if let Ok(board) = comms.board_bottom.try_recv() {
                    let mut d = menu.dom.lock().unwrap();
                    let Some(ui2::Node {
                        kind: ui2::Kind::Text { text, .. }, ..
                    }) = d.select_mut(0) else {continue;};
                    *text = format!(
                        "board1 = {:?}, board2 = {:?}",
                        board.hero_1_rfid, board.hero_2_rfid
                    );
                }
            }
        }
        Ok(())
    }

    fn on_remove(&self, _ctx: &mut engine::Context) -> Result<(), engine::Error> {
        Ok(())
    }
}
