mod builder;

pub mod components;
pub mod focus;
pub mod id_offset;
mod layout;
mod ui_context;
pub mod utils;

pub use builder::constructors;

pub type BoxedNode = builder::Box<builder::Node>;

use std::{path::PathBuf, rc::Rc};

use self::{
    layout::{CanCreateLayoutTree, LayoutTree, NoTransform},
    ui_context::UiContext,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InternalNodeId(u64);

impl InternalNodeId {
    pub fn from_u64(v: u64) -> Self {
        InternalNodeId(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeId(u64);

impl NodeId {
    pub fn from_u64(v: u64) -> Self {
        NodeId(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventId(u64);

impl EventId {
    pub fn from_u64(v: u64) -> Self {
        EventId(v)
    }
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Rect,
    Vert(Vec<InternalNodeId>),
    Hori(Vec<InternalNodeId>),
    Stack(Vec<InternalNodeId>),
    Text { text: String, font: PathBuf },
    Image(PathBuf),
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub kind: Kind,
    parent_id: Option<InternalNodeId>,
    user_id: Option<NodeId>,
    width: Option<i32>,
    height: Option<i32>,
    on_click: Option<EventId>,
    background_color: Option<(u8, u8, u8)>,
    color: Option<(u8, u8, u8)>,
    border_thickness: Option<i32>,
    padding: Option<i32>,
    gap: Option<i32>,
    border_color: Option<(u8, u8, u8)>,
    font_size: Option<u16>,
    visible: bool,
    focused: bool,
    focus_thickness: i32,
    focus_color: (u8, u8, u8),
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
    make_set_function!(set_gap, gap, i32);
    make_set_function!(set_font_size, font_size, u16);

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

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

type EventHandler = Rc<dyn Fn(&mut Dom, &mut engine::Context, InternalNodeId)>;

pub struct Dom {
    nodes: Vec<(InternalNodeId, Node)>,
    id_counter: u64,
    root_id: InternalNodeId,
    event_queue: Vec<(EventId, InternalNodeId)>,
    event_handlers: Vec<(EventId, EventHandler)>,
}

impl Dom {
    pub fn new(mut build: builder::Box<builder::Node>) -> Self {
        let mut nodes = Vec::new();
        let mut id_counter = 0;
        let root_id = build.build(
            &mut nodes,
            &mut id_counter,
            None,
            builder::DerivedProps::new(),
        );
        Self {
            nodes,
            root_id,
            id_counter,
            event_queue: Vec::new(),
            event_handlers: Vec::new(),
        }
    }

    pub fn add_event_handler<Id: Into<EventId>, F>(&mut self, event_id: Id, f: F)
    where
        F: Fn(&mut Dom, &mut engine::Context, InternalNodeId) + 'static,
    {
        self.event_handlers.push((event_id.into(), Rc::new(f)))
    }

    fn select_node<I>(&self, node_id: I) -> Option<&Node>
    where
        I: Into<InternalNodeId>,
    {
        let node_id = node_id.into();
        self.nodes
            .iter()
            .find(|(id, _)| *id == node_id)
            .map(|(_, node)| node)
    }

    fn select_node_mut<I>(&mut self, node_id: I) -> Option<&mut Node>
    where
        I: Into<InternalNodeId>,
    {
        let node_id = node_id.into();
        self.nodes
            .iter_mut()
            .find(|(id, _)| *id == node_id)
            .map(|(_, node)| node)
    }

    pub fn select<I>(&self, user_id: I) -> Option<&Node>
    where
        I: Into<NodeId>,
    {
        let user_id = user_id.into();
        let count = self
            .nodes
            .iter()
            .filter(|(_, node)| node.user_id.is_some_and(|id| id == user_id))
            .count();
        if count > 1 {
            println!("ui warning: colliding ids: {}", user_id.0);
        };
        self.nodes
            .iter()
            .find(|(_, node)| node.user_id.is_some_and(|id| id == user_id))
            .map(|(_, node)| node)
    }

    pub fn select_mut<I>(&mut self, user_id: I) -> Option<&mut Node>
    where
        I: Into<NodeId>,
    {
        let user_id = user_id.into();
        let count = self
            .nodes
            .iter()
            .filter(|(_, node)| node.user_id.is_some_and(|id| id == user_id))
            .count();
        if count > 1 {
            println!("ui warning: colliding ids: {}", user_id.0);
        };

        self.nodes
            .iter_mut()
            .find(|(_, node)| node.user_id.is_some_and(|id| id == user_id))
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

    fn build_layout_tree(&self, ctx: &mut impl UiContext) -> LayoutTree<'_> {
        self.nodes
            .iter()
            .find(|(id, _)| *id == self.root_id)
            .map(|(_, node)| node)
            .unwrap()
            .build_layout_tree(self.root_id, self, ctx, (0, 0), &mut NoTransform)
    }

    fn internal_ancestry_find_map<T, F: Fn(&Node) -> Option<T>>(
        &self,
        id: InternalNodeId,
        predicate: F,
    ) -> Option<T> {
        let node = self.select_node(id)?;
        match predicate(node) {
            None => self.internal_ancestry_find_map(node.parent_id?, predicate),
            rest => rest,
        }
    }

    pub fn ancestry_find_map<I, F, T>(&self, user_id: I, predicate: F) -> Option<T>
    where
        I: Into<NodeId>,
        F: Fn(&Node) -> Option<T>,
    {
        let node = self.select(user_id)?;
        match predicate(node) {
            None => self.internal_ancestry_find_map(node.parent_id?, predicate),
            rest => rest,
        }
    }

    fn click_node(&mut self, id: InternalNodeId) {
        if let Some((id, node)) = self.nodes.iter().find(|node| id == node.0) {
            if let Some(event_id) = node.on_click {
                self.event_queue.push((event_id, *id));
            }
        };
    }

    pub fn update(&mut self, ctx: &mut engine::Context) {
        let tree = self.build_layout_tree(ctx);
        tree.draw(ctx);
        if ctx.mouse_button_just_pressed(engine::MouseButton::Left) {
            if let Some(event) = tree.resolve_click(ctx.mouse_position()) {
                self.event_queue.push(event);
            }
        }
        self.handle_events(ctx);
    }
}
