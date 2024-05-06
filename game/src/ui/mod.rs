mod builder;

pub mod components;
mod layout;

pub use builder::constructors;
use engine::Context;

pub type BoxedNode = builder::Box<builder::Node>;

use std::{path::PathBuf, rc::Rc};

use self::layout::{CanCreateLayoutTree, LayoutTree, NoTransform};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeId(u64);

impl From<u64> for NodeId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserSpaceId(u64);

impl From<u64> for UserSpaceId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub enum Kind {
    Rect,
    Vert(Vec<NodeId>),
    Hori(Vec<NodeId>),
    Stack(Vec<NodeId>),
    Text { text: String, font: PathBuf },
    Image(PathBuf),
}

#[derive(Debug)]
pub struct Node {
    pub kind: Kind,
    id: Option<UserSpaceId>,
    width: Option<i32>,
    height: Option<i32>,
    on_click: Option<u64>,
    background_color: Option<(u8, u8, u8)>,
    color: Option<(u8, u8, u8)>,
    border_thickness: Option<i32>,
    padding: Option<i32>,
    border_color: Option<(u8, u8, u8)>,
    font_size: Option<u16>,
    visible: bool,
}

macro_rules! make_set_function {
    ($fid:ident, $id:ident, $type:ty) => {
        pub fn $fid(&mut self, $id: $type) {
            self.$id = Some($id);
        }
    };
}

impl Node {
    make_set_function!(set_width, width, i32);
    make_set_function!(set_height, height, i32);
    make_set_function!(set_background_color, background_color, (u8, u8, u8));
    make_set_function!(set_color, color, (u8, u8, u8));
    make_set_function!(set_border_color, border_color, (u8, u8, u8));
    make_set_function!(set_border_thickness, border_thickness, i32);
    make_set_function!(set_padding, padding, i32);
    make_set_function!(set_font_size, font_size, u16);

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn children<'dom>(&self, dom: &'dom Dom) -> Option<Vec<&'dom Node>> {
        match &self.kind {
            Kind::Vert(children) | Kind::Hori(children) | Kind::Stack(children) => {
                children.iter().map(|id| dom.select_node(*id)).collect()
            }
            _ => None,
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
        let root_id = build.build(&mut nodes, &mut id_counter, builder::DerivedProps::new());
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

    fn select_node<I>(&self, node_id: I) -> Option<&Node>
    where
        I: Into<NodeId>,
    {
        let node_id = node_id.into();
        self.nodes
            .iter()
            .find(|(id, _)| *id == node_id)
            .map(|(_, node)| node)
    }

    fn select_node_mut<I>(&mut self, node_id: I) -> Option<&mut Node>
    where
        I: Into<NodeId>,
    {
        let node_id = node_id.into();
        self.nodes
            .iter_mut()
            .find(|(id, _)| *id == node_id)
            .map(|(_, node)| node)
    }

    pub fn select<I>(&mut self, uid: I) -> Option<&Node>
    where
        I: Into<UserSpaceId>,
    {
        let uid = uid.into();
        let count = self
            .nodes
            .iter()
            .filter(|(_, node)| node.id.is_some_and(|id| id == uid))
            .count();
        if count > 1 {
            println!("ui warning: colliding ids: {}", uid.0);
        };
        self.nodes
            .iter()
            .find(|(_, node)| node.id.is_some_and(|id| id == uid))
            .map(|(_, node)| node)
    }

    pub fn select_mut<I>(&mut self, uid: I) -> Option<&mut Node>
    where
        I: Into<UserSpaceId>,
    {
        let uid = uid.into();
        let count = self
            .nodes
            .iter()
            .filter(|(_, node)| node.id.is_some_and(|id| id == uid))
            .count();
        if count > 1 {
            println!("ui warning: colliding ids: {}", uid.0);
        };

        self.nodes
            .iter_mut()
            .find(|(_, node)| node.id.is_some_and(|id| id == uid))
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

    fn build_layout_tree(&self, ctx: &mut Context) -> LayoutTree<'_> {
        self.nodes
            .iter()
            .find(|(id, _)| *id == self.root_id)
            .map(|(_, node)| node)
            .unwrap()
            .build_layout_tree(self, ctx, (0, 0), &mut NoTransform)
    }

    pub fn update(&mut self, ctx: &mut engine::Context) {
        let tree = self.build_layout_tree(ctx);
        tree.draw(ctx);
        if ctx.mouse_button_just_pressed(engine::MouseButton::Left) {
            if let Some(event_id) = tree.resolve_click(ctx.mouse_position()) {
                self.event_queue.push(event_id);
            }
        }
        self.handle_events(ctx);
    }
}
