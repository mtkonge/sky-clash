use std::{rc::Rc, sync::Mutex};

use super::{builder, constructors::Text, BoxedNode, Dom, Kind, Node, NodeId};

#[allow(non_snake_case)]
pub fn Button<S: Into<String>>(text: S) -> builder::Box<builder::Node> {
    Text(text)
        .with_padding(15)
        .with_border_thickness(2)
        .with_border_color((255, 255, 255))
}

type EventHandlerFn = dyn Fn(&mut Dom, &mut engine::Context);

pub struct ProgressBar {
    title: String,
    steps_filled: Rc<Mutex<i32>>,
    steps_total: i32,
    id_mask: u64,
    increase_handler: Option<Box<EventHandlerFn>>,
    decrease_handler: Option<Box<EventHandlerFn>>,
}

impl ProgressBar {
    pub fn new<S: Into<String>>(title: S, steps_total: i32, id_mask: u64) -> Self {
        Self {
            title: title.into(),
            steps_filled: Rc::new(Mutex::new(0)),
            steps_total,
            id_mask,
            increase_handler: None,
            decrease_handler: None,
        }
    }

    fn id(&self, id: u64) -> u64 {
        self.id_mask + id
    }

    pub fn steps_filled(&self) -> i32 {
        *self.steps_filled.lock().unwrap()
    }

    pub fn set_steps_filled(&mut self, steps_filled: i32) {
        *self.steps_filled.lock().unwrap() = steps_filled;
    }

    pub fn change_steps_filled(&mut self, delta: i32) {
        *self.steps_filled.lock().unwrap() += delta;
    }

    fn text(&self) -> String {
        format!(
            "{} ({:02}/{:02})",
            self.title,
            self.steps_filled(),
            self.steps_total
        )
    }

    pub fn build(&self) -> BoxedNode {
        use super::constructors::*;

        let middle = (self.steps_total / 2) as usize;
        let mut children: Vec<_> = (0..self.steps_total)
            .map(|i| {
                let color = if i < self.steps_filled() {
                    (255, 255, 255)
                } else {
                    (127, 127, 127)
                };

                Text("|").with_color(color).with_id(self.id(i as u64 + 1))
            })
            .collect();
        children.insert(
            middle,
            Text(self.text()).with_id(self.id(0)).with_padding(8),
        );

        Hori([
            Text("-").on_click(self.id(0)).with_padding(8),
            Hori(children),
            Text("+").on_click(self.id(1)).with_padding(8),
        ])
        .with_padding(8)
    }

    pub fn add_event_handlers(&self, dom: &mut Dom) {
        let steps_filled = self.steps_filled.clone();
        let steps_total = self.steps_total;
        dom.add_event_handler(
            self.id(0),
            move |_dom: &mut Dom, _ctx: &mut engine::Context, _id: NodeId| {
                if *steps_filled.lock().unwrap() == 0 {
                    return;
                }
                *steps_filled.lock().unwrap() -= 1;
            },
        );
        let steps_filled = self.steps_filled.clone();
        dom.add_event_handler(
            self.id(1),
            move |_dom: &mut Dom, _ctx: &mut engine::Context, _id: NodeId| {
                if *steps_filled.lock().unwrap() == steps_total {
                    return;
                }
                *steps_filled.lock().unwrap() += 1;
            },
        );
    }

    pub fn on_increase<F: Fn(&mut Dom, &mut engine::Context) + 'static>(&mut self, f: F) {
        self.increase_handler = Some(Box::new(f));
    }

    pub fn on_decrease<F: Fn(&mut Dom, &mut engine::Context) + 'static>(&mut self, f: F) {
        self.decrease_handler = Some(Box::new(f));
    }

    pub fn update(&mut self, dom: &mut Dom) {
        let Some(Node {
            kind: Kind::Text {
                text: node_text, ..
            },
            ..
        }) = dom.select_mut(self.id(0))
        else {
            panic!("not found >_<")
        };
        *node_text = self.text();

        for i in 0..self.steps_total {
            let Some(node) = dom.select_mut(self.id(i as u64 + 1)) else {
                panic!("percentage does not exist");
            };
            if i < self.steps_filled() {
                node.set_color((255, 255, 255));
            } else {
                node.set_color((127, 127, 127));
            }
        }
    }
}
