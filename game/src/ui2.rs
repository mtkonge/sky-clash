use std::{path::PathBuf, rc::Rc};

use crate::engine;

#[derive(Clone, Copy, PartialEq)]
pub struct NodeId(u64);

impl From<u64> for NodeId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

pub enum Kind {
    Vert(Vec<NodeId>),
    Hori(Vec<NodeId>),
    Text {
        text: String,
        font: PathBuf,
        size: u16,
    },
}

pub struct Node {
    pub kind: Kind,
    id: Option<u64>,
    width: Option<i32>,
    height: Option<i32>,
    on_click: Option<u64>,
}

impl Node {
    pub fn children<'dom>(&self, dom: &'dom Dom) -> Option<Vec<&'dom Node>> {
        match &self.kind {
            Kind::Vert(children) | Kind::Hori(children) => {
                children.iter().map(|id| dom.select(*id)).collect()
            }
            Kind::Text { .. } => None,
        }
    }

    pub fn size(&self, dom: &Dom, ctx: &mut engine::Context) -> (i32, i32) {
        match &self.kind {
            node @ (Kind::Vert(_) | Kind::Hori(_)) => {
                let children = self.children(dom).unwrap();
                let mut size = (0, 0);
                let mut max_size = (0, 0);
                for child in children {
                    let child_size = child.size(dom, ctx);
                    size.0 += child_size.0;
                    size.1 += child_size.1;
                    max_size = (
                        std::cmp::max(child_size.0, max_size.0),
                        std::cmp::max(child_size.1, max_size.1),
                    );
                }
                match node {
                    Kind::Vert(_) => (max_size.0, size.1),
                    Kind::Hori(_) => (size.0, max_size.1),
                    Kind::Text { .. } => unreachable!(),
                }
            }
            Kind::Text { text, font, size } => {
                let font_id = ctx.load_font(font, *size).unwrap();
                let (w, h) = ctx.text_size(font_id, text).unwrap();
                (w as i32, h as i32)
            }
        }
    }

    pub fn draw(&self, dom: &Dom, ctx: &mut engine::Context, pos: (i32, i32)) {
        let size = self.size(dom, ctx);
        match &self.kind {
            Kind::Vert(_) => {
                let children = self.children(dom).unwrap();
                let mut pos = pos;
                for child in children {
                    let child_size = child.size(dom, ctx);
                    let x = pos.0 + (size.0 - child_size.0) / 2;
                    let y = pos.1;
                    pos.1 += child_size.1;
                    child.draw(dom, ctx, (x, y));
                }
            }
            Kind::Hori(_) => {
                let children = self.children(dom).unwrap();
                let mut pos = pos;
                for child in children {
                    let child_size = child.size(dom, ctx);
                    let x = pos.0;
                    let y = pos.1 + (size.1 - child_size.1) / 2;
                    pos.0 += child_size.0;
                    child.draw(dom, ctx, (x, y));
                }
            }
            Kind::Text { text, size, font } => {
                let font_id = ctx.load_font(font, *size).unwrap();
                let text = ctx.render_text(font_id, text, (255, 255, 255)).unwrap();
                ctx.draw_texture(text.texture, pos.0, pos.1).unwrap();
            }
        }
    }
}

type EventHandler = Rc<dyn Fn(&mut Dom, &mut engine::Context, NodeId)>;

pub struct Dom {
    nodes: Vec<(NodeId, Node)>,
    id_counter: u64,
    root_id: NodeId,
    event_queue: Vec<(u64, NodeId)>,
    event_handlers: Vec<(u64, EventHandler)>,
}

impl Dom {
    pub fn new(mut build: builder::Box<builder::Node>) -> Self {
        let mut nodes = Vec::new();
        let mut id_counter = 0;
        let root_id = build.build(&mut nodes, &mut id_counter);
        Self {
            nodes,
            root_id,
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

    pub fn select<I>(&self, node_id: I) -> Option<&Node>
    where
        I: Into<NodeId>,
    {
        let node_id = node_id.into();
        self.nodes
            .iter()
            .find(|(id, _)| *id == node_id)
            .map(|(_, node)| node)
    }

    pub fn select_mut<I>(&mut self, node_id: I) -> Option<&mut Node>
    where
        I: Into<NodeId>,
    {
        let node_id = node_id.into();
        self.nodes
            .iter_mut()
            .find(|(id, _)| *id == node_id)
            .map(|(_, node)| node)
    }

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

    pub fn resolve_click(&mut self, pos: (i32, i32)) {}

    pub fn draw(&self, ctx: &mut engine::Context, pos: (i32, i32)) {
        self.nodes
            .iter()
            .find(|(id, _)| *id == self.root_id)
            .map(|(_, node)| node)
            .expect("")
            .draw(self, ctx, pos);
    }

    pub fn update(&mut self, ctx: &mut engine::Context) {
        if ctx.mouse_button_pressed(engine::MouseButton::Left) {
            self.resolve_click(ctx.mouse_position())
        }
        self.handle_events(ctx);
        self.draw(ctx, (0, 0));
    }
}

pub mod builder {
    use super::NodeId;
    use std::{
        boxed::Box as InnerBox,
        ops::{Deref, DerefMut},
        path::PathBuf,
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
                            kind: super::Kind::Text {
                                text: v.clone(),
                                font: PathBuf::from("textures/ttf/OpenSans.ttf"),
                                size: 16,
                            },
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
