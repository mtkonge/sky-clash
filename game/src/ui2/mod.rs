mod builder;
pub use builder::constructors;

use std::{path::PathBuf, rc::Rc};

#[derive(Clone, Copy, PartialEq)]
pub struct NodeId(u64);

impl From<u64> for NodeId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

pub enum Kind {
    Rect,
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
    background_color: Option<(u8, u8, u8)>,
    color: Option<(u8, u8, u8)>,
    border_thickness: Option<i32>,
    padding: Option<i32>,
    border_color: Option<(u8, u8, u8)>,
}

impl Node {
    pub fn children<'dom>(&self, dom: &'dom Dom) -> Option<Vec<&'dom Node>> {
        match &self.kind {
            Kind::Vert(children) | Kind::Hori(children) => {
                children.iter().map(|id| dom.select(*id)).collect()
            }
            _ => None,
        }
    }

    pub fn size(&self, dom: &Dom, ctx: &mut engine::Context) -> (i32, i32) {
        let padding = (self.padding.unwrap_or(0) + self.border_thickness.unwrap_or(0)) * 2;
        match &self.kind {
            Kind::Rect => (
                self.width.unwrap_or(0) + padding,
                self.height.unwrap_or(0) + padding,
            ),
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
                let calculated_size = match node {
                    Kind::Vert(_) => (max_size.0, size.1),
                    Kind::Hori(_) => (size.0, max_size.1),
                    _ => unreachable!(),
                };
                if self.width.is_some_and(|w| w < calculated_size.0)
                    || self.width.is_some_and(|h| h < calculated_size.0)
                {
                    panic!("overflow >~<");
                }
                (
                    self.width.unwrap_or(calculated_size.0) + padding,
                    self.height.unwrap_or(calculated_size.1) + padding,
                )
            }
            Kind::Text { text, font, size } => {
                let font_id = ctx.load_font(font, *size).unwrap();
                let (w, h) = ctx.text_size(font_id, text).unwrap();
                (w as i32 + padding, h as i32 + padding)
            }
        }
    }

    pub fn draw(&self, dom: &Dom, ctx: &mut engine::Context, pos: (i32, i32)) {
        let size = self.size(dom, ctx);
        if let Some(color) = self.background_color {
            ctx.draw_rect(color, pos.0, pos.1, size.0 as u32, size.1 as u32)
                .unwrap();
        }
        let offset = self.padding.unwrap_or(0) + self.border_thickness.unwrap_or(0);
        match &self.kind {
            Kind::Rect => {}
            Kind::Vert(_) => {
                let children = self.children(dom).unwrap();
                let mut pos = pos;
                for child in children {
                    let child_size = child.size(dom, ctx);
                    let x = pos.0 + (size.0 - child_size.0) / 2;
                    let y = pos.1;
                    pos.1 += child_size.1;
                    child.draw(dom, ctx, (x + offset, y + offset));
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
                    child.draw(dom, ctx, (x + offset, y + offset));
                }
            }
            Kind::Text { text, size, font } => {
                let font_id = ctx.load_font(font, *size).unwrap();
                let text = ctx
                    .render_text(font_id, text, self.color.unwrap_or((255, 255, 255)))
                    .unwrap();
                ctx.draw_texture(text.texture, pos.0 + offset, pos.1 + offset)
                    .unwrap();
            }
        }
        if let Some(thickness) = self.border_thickness {
            let thickness = thickness as u32;
            let border_color = self.border_color.unwrap_or((255, 255, 255));
            ctx.draw_rect(border_color, pos.0, pos.1, size.0 as u32, thickness)
                .unwrap();
            ctx.draw_rect(border_color, pos.0, pos.1, thickness, size.1 as u32)
                .unwrap();
            ctx.draw_rect(
                border_color,
                pos.0 + size.0 - thickness as i32,
                pos.1,
                thickness,
                size.1 as u32,
            )
            .unwrap();
            ctx.draw_rect(
                border_color,
                pos.0,
                pos.1 + size.1 - thickness as i32,
                size.0 as u32,
                thickness,
            )
            .unwrap();
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
