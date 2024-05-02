mod builder;

pub mod components;

pub use builder::constructors;
use engine::Context;

pub type BoxedNode = builder::Box<builder::Node>;

use std::{path::PathBuf, rc::Rc};

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
pub struct LayoutTreeLeaf<'a> {
    size: (i32, i32),
    pos: (i32, i32),
    inner: &'a Node,
}

#[derive(Debug)]
pub enum LayoutTree<'a> {
    Single(LayoutTreeLeaf<'a>),
    Multiple(LayoutTreeLeaf<'a>, Vec<LayoutTree<'a>>),
}

impl LayoutTree<'_> {
    pub fn draw(&self, ctx: &mut Context) {
        match self {
            LayoutTree::Single(leaf) => leaf.draw(ctx),
            LayoutTree::Multiple(leaf, children) => {
                leaf.draw(ctx);
                for tree in children {
                    tree.draw(ctx);
                }
            }
        }
    }
}

impl LayoutTreeLeaf<'_> {
    fn draw_border(&self, ctx: &mut Context) {
        let pos = self.pos;
        let size = (self.size.0 as u32, self.size.1 as u32);
        if let Some(thickness) = self.inner.border_thickness {
            let thickness = thickness as u32;
            let border_color = self.inner.border_color.unwrap_or((255, 255, 255));
            ctx.draw_rect(border_color, pos.0, pos.1, size.0, thickness)
                .unwrap();
            ctx.draw_rect(border_color, pos.0, pos.1, thickness, size.1)
                .unwrap();
            ctx.draw_rect(
                border_color,
                pos.0 + size.0 as i32 - thickness as i32,
                pos.1,
                thickness,
                size.1,
            )
            .unwrap();
            ctx.draw_rect(
                border_color,
                pos.0,
                pos.1 + size.1 as i32 - thickness as i32,
                size.0,
                thickness,
            )
            .unwrap();
        }
    }
    pub fn draw(&self, ctx: &mut Context) {
        if !self.inner.visible {
            return;
        }
        if let Some(color) = self.inner.background_color {
            ctx.draw_rect(
                color,
                self.pos.0,
                self.pos.1,
                self.size.0 as u32,
                self.size.1 as u32,
            )
            .unwrap();
        }

        match &self.inner.kind {
            Kind::Vert(_) | Kind::Hori(_) | Kind::Stack(_) | Kind::Rect => { /* do nothing */ }
            Kind::Text { text, font } => {
                let font_size = self.inner.font_size.unwrap_or(15);
                let font_id = ctx.load_font(font, font_size).unwrap();
                let text = ctx
                    .render_text(font_id, text, self.inner.color.unwrap_or((255, 255, 255)))
                    .unwrap();
                let offset =
                    self.inner.padding.unwrap_or(0) + self.inner.border_thickness.unwrap_or(0);
                ctx.draw_texture(text.texture, self.pos.0 + offset, self.pos.1 + offset)
                    .unwrap();
            }
            Kind::Image(src) => {
                let texture = ctx.load_texture(src).unwrap();
                let texture_size = ctx.texture_size(texture).unwrap();
                let offset =
                    self.inner.padding.unwrap_or(0) + self.inner.border_thickness.unwrap_or(0);
                ctx.draw_texture_sized(
                    texture,
                    self.pos.0 + offset,
                    self.pos.1 + offset,
                    self.inner.width.unwrap_or(texture_size.0 as i32) as u32,
                    self.inner.height.unwrap_or(texture_size.1 as i32) as u32,
                )
                .unwrap();
            }
        }

        self.draw_border(ctx);
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

trait TransformersRobotsInDisguise {
    fn pos(&mut self, size: (i32, i32)) -> (i32, i32);
}

struct HoriTransform {
    acc: i32,
    content_size: (i32, i32),
    padding: i32,
}
impl HoriTransform {
    fn new(content_size: (i32, i32), padding: i32) -> Self {
        Self {
            acc: 0,
            content_size,
            padding,
        }
    }
}
impl TransformersRobotsInDisguise for HoriTransform {
    fn pos(&mut self, child_size: (i32, i32)) -> (i32, i32) {
        let x = self.acc;
        let y = (self.content_size.1 - child_size.1) / 2;
        self.acc += child_size.0;
        (x + self.padding, y + self.padding)
    }
}

struct VertTransform {
    acc: i32,
    content_size: (i32, i32),
    padding: i32,
}
impl VertTransform {
    fn new(content_size: (i32, i32), padding: i32) -> Self {
        Self {
            acc: 0,
            content_size,
            padding,
        }
    }
}
impl TransformersRobotsInDisguise for VertTransform {
    fn pos(&mut self, child_size: (i32, i32)) -> (i32, i32) {
        let x = (self.content_size.0 - child_size.0) / 2;
        let y = self.acc;
        self.acc += child_size.1;
        (x + self.padding, y + self.padding)
    }
}

struct StackTransform {
    content_size: (i32, i32),
    padding: i32,
}
impl StackTransform {
    fn new(content_size: (i32, i32), padding: i32) -> Self {
        Self {
            content_size,
            padding,
        }
    }
}
impl TransformersRobotsInDisguise for StackTransform {
    fn pos(&mut self, child_size: (i32, i32)) -> (i32, i32) {
        let x = (self.content_size.0 - child_size.0) / 2;
        let y = (self.content_size.1 - child_size.1) / 2;
        (x + self.padding, y + self.padding)
    }
}

struct NoTransform;
impl TransformersRobotsInDisguise for NoTransform {
    fn pos(&mut self, _child_size: (i32, i32)) -> (i32, i32) {
        (0, 0)
    }
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

    fn build_layout_tree<'dom>(
        &'dom self,
        dom: &'dom Dom,
        ctx: &mut engine::Context,
        parent_pos: (i32, i32),
        pos_transformer: &mut impl TransformersRobotsInDisguise,
    ) -> LayoutTree<'dom> {
        if !self.visible {
            return LayoutTree::Single(LayoutTreeLeaf {
                size: (0, 0),
                pos: (0, 0),
                inner: self,
            });
        }
        fn build_leaf<'a>(
            node: &'a Node,
            pos_offset: &mut impl TransformersRobotsInDisguise,
            parent_pos: (i32, i32),
            default_size: (i32, i32),
        ) -> LayoutTreeLeaf<'a> {
            let padding = node.padding.unwrap_or(0) + node.border_thickness.unwrap_or(0);
            let size = (
                node.width.unwrap_or(default_size.0) + padding * 2,
                node.height.unwrap_or(default_size.1) + padding * 2,
            );
            let pos = pos_offset.pos(size);
            let pos = (pos.0 + parent_pos.0, pos.1 + parent_pos.1);
            LayoutTreeLeaf {
                size,
                pos,
                inner: node,
            }
        }

        match &self.kind {
            Kind::Text { text, font } => {
                let font_size = self.font_size.unwrap_or(15);
                let font_id = ctx.load_font(font, font_size).unwrap();
                let size = ctx.text_size(font_id, text).unwrap();
                let size = (size.0 as i32, size.1 as i32);
                let leaf = build_leaf(self, pos_transformer, parent_pos, size);
                LayoutTree::Single(leaf)
            }
            Kind::Rect | Kind::Image(_) => {
                let leaf = build_leaf(self, pos_transformer, parent_pos, (0, 0));
                LayoutTree::Single(leaf)
            }
            Kind::Hori(children) => {
                let padding = self.padding.unwrap_or(0) + self.border_thickness.unwrap_or(0);

                let calc_content_size = |acc: (i32, i32), curr: LayoutTree| {
                    let (LayoutTree::Single(leaf) | LayoutTree::Multiple(leaf, _)) = curr;
                    (acc.0 + leaf.size.0, std::cmp::max(acc.1, leaf.size.1))
                };

                let mut content_size = (0, 0);
                for node in children {
                    let node = dom.select_node(*node).unwrap();
                    let node = node.build_layout_tree(dom, ctx, parent_pos, &mut NoTransform);
                    content_size = calc_content_size(content_size, node);
                }

                let pos = pos_transformer.pos(content_size);
                let pos = (pos.0 + parent_pos.0, pos.1 + parent_pos.1);

                let mut new_children = Vec::new();
                let mut transformer = HoriTransform::new(content_size, padding);
                for node in children {
                    let node = dom.select_node(*node).unwrap();
                    let node = node.build_layout_tree(dom, ctx, pos, &mut transformer);
                    new_children.push(node);
                }

                let leaf = LayoutTreeLeaf {
                    size: (content_size.0 + padding, content_size.1 + padding),
                    pos,
                    inner: self,
                };

                LayoutTree::Multiple(leaf, new_children)
            }
            Kind::Vert(children) => {
                let padding = self.padding.unwrap_or(0) + self.border_thickness.unwrap_or(0);

                let calc_content_size = |acc: (i32, i32), curr: LayoutTree| {
                    let (LayoutTree::Single(leaf) | LayoutTree::Multiple(leaf, _)) = curr;
                    (std::cmp::max(acc.0, leaf.size.0), acc.1 + leaf.size.1)
                };

                let mut content_size = (0, 0);
                for node in children {
                    let node = dom.select_node(*node).unwrap();
                    let node = node.build_layout_tree(dom, ctx, parent_pos, &mut NoTransform);
                    content_size = calc_content_size(content_size, node);
                }

                let pos = pos_transformer.pos(content_size);
                let pos = (pos.0 + parent_pos.0, pos.1 + parent_pos.1);

                let mut transformer = VertTransform::new(content_size, padding);
                let children: Vec<LayoutTree<'dom>> = children
                    .iter()
                    .filter_map(|i| dom.select_node(*i))
                    .map(|node| node.build_layout_tree(dom, ctx, pos, &mut transformer))
                    .collect();

                let leaf = LayoutTreeLeaf {
                    size: (content_size.0 + padding, content_size.1 + padding),
                    pos,
                    inner: self,
                };

                LayoutTree::Multiple(leaf, children)
            }
            Kind::Stack(children) => {
                let padding = self.padding.unwrap_or(0) + self.border_thickness.unwrap_or(0);

                let calc_content_size = |acc: (i32, i32), curr: LayoutTree| {
                    let (LayoutTree::Single(leaf) | LayoutTree::Multiple(leaf, _)) = curr;
                    (
                        std::cmp::max(acc.0, leaf.size.0),
                        std::cmp::max(acc.1, leaf.size.1),
                    )
                };

                let content_size = children
                    .iter()
                    .filter_map(|i| dom.select_node(*i))
                    .map(|node| node.build_layout_tree(dom, ctx, (0, 0), &mut NoTransform))
                    .fold((0, 0), calc_content_size);

                let pos = pos_transformer.pos(content_size);
                let pos = (pos.0 + parent_pos.0, pos.1 + parent_pos.1);

                let mut transformer = StackTransform::new(content_size, padding);
                let children: Vec<LayoutTree<'dom>> = children
                    .iter()
                    .filter_map(|i| dom.select_node(*i))
                    .map(|node| node.build_layout_tree(dom, ctx, pos, &mut transformer))
                    .collect();

                let w = self.width.unwrap_or(content_size.0 + padding);
                let h = self.height.unwrap_or(content_size.1 + padding);

                let leaf = LayoutTreeLeaf {
                    size: (w, h),
                    pos,
                    inner: self,
                };

                LayoutTree::Multiple(leaf, children)
            }
        }
    }

    // pub fn draw(&self, dom: &Dom, ctx: &mut engine::Context, pos: (i32, i32)) {
    //     if !self.visible {
    //         return;
    //     }
    //     let size = self.size(dom, ctx);
    //     if let Some(color) = self.background_color {
    //         ctx.draw_rect(color, pos.0, pos.1, size.0 as u32, size.1 as u32)
    //             .unwrap();
    //     }
    //     let offset = self.padding.unwrap_or(0) + self.border_thickness.unwrap_or(0);
    //     match &self.kind {
    //     }
    // }
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
        dbg!(&tree);
        tree.draw(ctx);
        // if ctx.mouse_button_just_pressed(engine::MouseButton::Left) {
        //     self.resolve_click(ctx, ctx.mouse_position())
        // }
        // self.handle_events(ctx);
        // self.draw(ctx, (0, 0));
    }
}
