use std::rc::Rc;

use crate::engine;

#[derive(Clone, Copy, PartialEq)]
pub struct NodeId(u64);

pub enum Kind {
    Vert(Vec<NodeId>),
    Hori(Vec<NodeId>),
    Text(String),
}

pub struct Node {
    kind: Kind,
    id: Option<u64>,
    width: Option<i32>,
    height: Option<i32>,
    on_click: Option<u64>,
}

type EventHandler = Rc<dyn Fn(&mut Dom, &mut engine::Context, NodeId)>;

pub struct Dom {
    nodes: Vec<(NodeId, Node)>,
    id_counter: u64,
    event_queue: Vec<(u64, NodeId)>,
    event_handlers: Vec<(u64, EventHandler)>,
}

impl Dom {
    pub fn new(mut build: builder::Box<builder::Node>) -> Self {
        let mut nodes = Vec::new();
        let mut id_counter = 0;
        build.build(&mut nodes, &mut id_counter);
        Self {
            nodes,
            id_counter,
            event_queue: Vec::new(),
            event_handlers: Vec::new(),
        }
    }

    pub fn add_event_handler<F>(&mut self, event_id: u64, f: F)
    where
        F: Fn(&mut Dom, &mut engine::Context, NodeId) + 'static,
    {
        self.event_handlers.push((event_id, Rc::new(f)))
    }

    pub fn select(&mut self, node_id: NodeId) -> Option<&mut Node> {
        self.nodes
            .iter_mut()
            .find(|(id, _)| *id == node_id)
            .map(|(_, node)| node)
    }

    pub fn resolve_click(&mut self, pos: (i32, i32)) {}

    pub fn handle_events(&mut self, ctx: &mut engine::Context) {
        let drained = std::mem::take(&mut self.event_queue);
        for (event_id, node_id) in drained {
            for (event_id_candidate, f) in self.event_handlers.clone() {
                if event_id_candidate == event_id {
                    f(self, ctx, node_id);
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut engine::Context) {}
}

pub mod builder {
    use super::NodeId;
    use std::{
        boxed::Box as InnerBox,
        ops::{Deref, DerefMut},
    };

    pub struct Box<T> {
        inner: InnerBox<T>,
    }

    impl<T> Box<T> {
        pub fn new(v: T) -> Self {
            Self {
                inner: InnerBox::new(v),
            }
        }
    }

    impl<T> Deref for Box<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            self.inner.deref()
        }
    }

    impl<T> DerefMut for Box<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.inner.deref_mut()
        }
    }

    pub enum Kind {
        Vert(Vec<Box<Node>>),
        Hori(Vec<Box<Node>>),
        Text(String),
    }

    pub mod constructors {
        #![allow(non_snake_case)]
        use super::*;
        pub fn Vert<I: IntoIterator<Item = Box<Node>>>(nodes: I) -> Box<Node> {
            Kind::Vert(nodes.into_iter().collect()).into()
        }
        pub fn Hori<I: IntoIterator<Item = Box<Node>>>(nodes: I) -> Box<Node> {
            Kind::Hori(nodes.into_iter().collect()).into()
        }
        pub fn Text<S: Into<String>>(text: S) -> Box<Node> {
            Kind::Text(text.into()).into()
        }
    }

    pub struct Node {
        kind: Kind,
        id: Option<u64>,
        width: Option<i32>,
        height: Option<i32>,
        on_click: Option<u64>,
    }

    impl Node {
        pub fn new(kind: Kind) -> Box<Node> {
            Box::new(Self {
                kind,
                id: None,
                width: None,
                height: None,
                on_click: None,
            })
        }

        pub fn build(
            &mut self,
            nodes: &mut Vec<(NodeId, super::Node)>,
            id_counter: &mut u64,
        ) -> super::NodeId {
            let id = super::NodeId(*id_counter);
            *id_counter += 1;
            match &mut self.kind {
                Kind::Vert(ref mut children) => {
                    let mut children_ids = Vec::new();
                    for mut child in children.drain(..) {
                        children_ids.push(child.build(nodes, id_counter));
                    }
                    nodes.push((
                        id,
                        super::Node {
                            kind: super::Kind::Vert(children_ids),
                            id: self.id,
                            width: self.width,
                            height: self.height,
                            on_click: self.on_click,
                        },
                    ));
                }
                Kind::Hori(ref mut children) => {
                    let mut children_ids = Vec::new();
                    for mut child in children.drain(..) {
                        children_ids.push(child.build(nodes, id_counter));
                    }
                    nodes.push((
                        id,
                        super::Node {
                            kind: super::Kind::Hori(children_ids),
                            id: self.id,
                            width: self.width,
                            height: self.height,
                            on_click: self.on_click,
                        },
                    ));
                }
                Kind::Text(v) => {
                    nodes.push((
                        id,
                        super::Node {
                            kind: super::Kind::Text(v.clone()),
                            id: self.id,
                            width: self.width,
                            height: self.height,
                            on_click: self.on_click,
                        },
                    ));
                }
            }
            id
        }
    }

    impl Box<Node> {
        pub fn with_id(mut self, id: u64) -> Self {
            self.id = Some(id);
            self
        }
        pub fn with_width(mut self, width: i32) -> Self {
            self.width = Some(width);
            self
        }
        pub fn with_height(mut self, height: i32) -> Self {
            self.height = Some(height);
            self
        }
    }

    impl From<Kind> for Box<Node> {
        fn from(value: Kind) -> Self {
            Node::new(value)
        }
    }
}
