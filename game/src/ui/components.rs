use crate::shared_ptr::SharedPtr;

use super::{builder, constructors::Text, BoxedNode, Dom, Kind, Node, NodeId};

#[allow(non_snake_case)]
pub fn Button<S: Into<String>>(text: S) -> builder::Box<builder::Node> {
    Text(text)
        .with_padding(15)
        .with_border_thickness(2)
        .with_border_color((255, 255, 255))
}

type Int = i64;

pub struct ProgressBar {
    title: String,
    filled: SharedPtr<Int>,
    total: Int,
    lower_limit: SharedPtr<Int>,
    upper_limit: SharedPtr<Int>,
    id_mask: u64,
}

impl ProgressBar {
    pub fn new<S: Into<String>>(title: S, steps_total: Int, id_mask: u64) -> Self {
        Self {
            title: title.into(),
            filled: SharedPtr::new(0),
            total: steps_total,
            lower_limit: SharedPtr::new(0),
            upper_limit: SharedPtr::new(steps_total),
            id_mask,
        }
    }

    fn id(&self, id: u64) -> u64 {
        self.id_mask + id
    }

    pub fn steps_filled(&self) -> Int {
        *self.filled.lock()
    }

    pub fn set_steps_filled(&mut self, steps_filled: Int) -> &mut Self {
        *self.filled.lock() = steps_filled;
        self
    }

    pub fn change_steps_filled(&mut self, delta: Int) -> &mut Self {
        *self.filled.lock() += delta;
        self
    }

    pub fn set_lower_limit(&mut self, limit: Int) -> &mut Self {
        if self.steps_filled() < limit {
            self.set_steps_filled(limit);
        }
        self
    }

    pub fn set_upper_limit(&mut self, limit: Int) -> &mut Self {
        *self.upper_limit.lock() = limit;
        if self.steps_filled() > limit {
            self.set_steps_filled(limit);
        }
        self
    }

    fn text(&self) -> String {
        format!(
            "{} ({:02}/{:02})",
            self.title,
            self.steps_filled(),
            self.total
        )
    }

    pub fn build(&self) -> BoxedNode {
        use super::constructors::*;

        let middle = (self.total / 2) as usize;
        let mut children: Vec<_> = (0..self.total)
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
            Vert([Text(self.text()).with_id(self.id(0))]).with_width(130),
        );

        Hori([
            Text("-").on_click(self.id(0)).with_padding(8),
            Hori(children),
            Text("+").on_click(self.id(1)).with_padding(8),
        ])
        .with_padding(8)
    }

    pub fn add_event_handlers(&self, dom: &mut Dom) {
        let steps_filled = self.filled.clone();
        let steps_total = self.total;
        let lower_limit = self.lower_limit.clone();
        let upper_limit = self.upper_limit.clone();
        dom.add_event_handler(
            self.id(0),
            move |_dom: &mut Dom, _ctx: &mut engine::Context, _id: NodeId| {
                if *steps_filled.lock() == 0 {
                    return;
                }
                if *steps_filled.lock() <= *lower_limit.lock() {
                    return;
                }
                *steps_filled.lock() -= 1;
            },
        );
        let steps_filled = self.filled.clone();
        dom.add_event_handler(
            self.id(1),
            move |_dom: &mut Dom, _ctx: &mut engine::Context, _id: NodeId| {
                if *steps_filled.lock() == steps_total {
                    return;
                }
                if *steps_filled.lock() >= *upper_limit.lock() {
                    return;
                }
                *steps_filled.lock() += 1;
            },
        );
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

        for i in 0..self.total {
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
