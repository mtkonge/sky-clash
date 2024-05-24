use super::{Dom, EventId, InternalNodeId, NodeId};
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
        &*self.inner
    }
}

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

pub enum Kind {
    Rect,
    Vert(Vec<Box<Node>>),
    Hori(Vec<Box<Node>>),
    Text(String),
    Image(PathBuf),
    Stack(Vec<Box<Node>>),
}

pub mod constructors {
    #![allow(non_snake_case)]
    use super::{Box, Kind, Node, PathBuf};
    pub fn Rect() -> Box<Node> {
        Kind::Rect.into()
    }
    pub fn Vert<I: IntoIterator<Item = Box<Node>>>(nodes: I) -> Box<Node> {
        Kind::Vert(nodes.into_iter().collect()).into()
    }
    pub fn Hori<I: IntoIterator<Item = Box<Node>>>(nodes: I) -> Box<Node> {
        Kind::Hori(nodes.into_iter().collect()).into()
    }
    pub fn Stack<I: IntoIterator<Item = Box<Node>>>(nodes: I) -> Box<Node> {
        Kind::Stack(nodes.into_iter().collect()).into()
    }
    pub fn Text<S: Into<String>>(text: S) -> Box<Node> {
        Kind::Text(text.into()).into()
    }
    pub fn Image<P: Into<PathBuf>>(path: P) -> Box<Node> {
        Kind::Image(path.into()).into()
    }
}

#[derive(Clone)]
pub struct DerivedProps {
    color: Option<(u8, u8, u8)>,
}

impl DerivedProps {
    pub fn new() -> Self {
        Self { color: None }
    }
}

pub struct Node {
    kind: Kind,
    id: Option<NodeId>,
    width: Option<i32>,
    height: Option<i32>,
    on_click: Option<EventId>,
    background_color: Option<(u8, u8, u8)>,
    color: Option<(u8, u8, u8)>,
    border_thickness: Option<i32>,
    border_color: Option<(u8, u8, u8)>,
    padding: Option<i32>,
    font_size: Option<u16>,
    visible: bool,
    gap: Option<i32>,
}

impl Node {
    pub fn new(kind: Kind) -> Box<Node> {
        Box::new(Self {
            kind,
            id: None,
            width: None,
            height: None,
            on_click: None,
            background_color: None,
            color: None,
            border_thickness: None,
            border_color: None,
            padding: None,
            font_size: None,
            gap: None,
            visible: true,
        })
    }

    pub fn build_from_dom(&mut self, dom: &mut Dom) -> super::InternalNodeId {
        self.build(
            &mut dom.nodes,
            &mut dom.id_counter,
            None,
            DerivedProps::new(),
        )
    }

    pub fn build(
        &mut self,
        nodes: &mut Vec<(InternalNodeId, super::Node)>,
        id_counter: &mut u64,
        parent_id: Option<InternalNodeId>,
        derived_props: DerivedProps,
    ) -> super::InternalNodeId {
        let derived_props = DerivedProps {
            color: derived_props.color.or(self.color),
        };

        let id = super::InternalNodeId(*id_counter);
        *id_counter += 1;
        let kind = match &mut self.kind {
            Kind::Rect => super::Kind::Rect,
            Kind::Vert(ref mut children)
            | Kind::Hori(ref mut children)
            | Kind::Stack(ref mut children) => {
                let mut children_ids = Vec::new();
                for mut child in children.drain(..) {
                    children_ids.push(child.build(
                        nodes,
                        id_counter,
                        Some(id),
                        derived_props.clone(),
                    ));
                }
                match self.kind {
                    Kind::Vert(_) => super::Kind::Vert(children_ids),
                    Kind::Hori(_) => super::Kind::Hori(children_ids),
                    Kind::Stack(_) => super::Kind::Stack(children_ids),
                    _ => unreachable!(),
                }
            }
            Kind::Text(v) => super::Kind::Text {
                text: v.clone(),
                font: PathBuf::from("textures/ttf/OpenSans.ttf"),
            },
            Kind::Image(src) => super::Kind::Image(src.clone()),
        };
        nodes.push((
            id,
            super::Node {
                kind,
                parent_id,
                user_id: self.id,
                width: self.width.map(|v| f64::from(v)),
                height: self.height.map(|v| f64::from(v)),
                on_click: self.on_click,
                background_color: self.background_color,
                color: self.color,
                gap: self.gap.map(|v| f64::from(v)),
                border_color: self.border_color,
                border_thickness: self.border_thickness.map(|v| f64::from(v)),
                padding: self.padding.map(|v| f64::from(v)),
                font_size: self.font_size,
                visible: self.visible,
                focused: false,
                focus_color: (53, 73, 136),
                focus_thickness: 4.0,
            },
        ));
        id
    }
}

macro_rules! make_with_function {
    ($fid:ident, $id:ident, $type:ty) => {
        pub fn $fid(mut self, $id: $type) -> Self {
            self.$id = Some($id);
            self
        }
    };
}

impl Box<Node> {
    make_with_function!(width, width, i32);
    make_with_function!(height, height, i32);
    make_with_function!(background_color, background_color, (u8, u8, u8));
    make_with_function!(color, color, (u8, u8, u8));
    make_with_function!(border_color, border_color, (u8, u8, u8));
    make_with_function!(border_thickness, border_thickness, i32);
    make_with_function!(padding, padding, i32);
    make_with_function!(gap, gap, i32);
    make_with_function!(font_size, font_size, u16);

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn id<T: Into<NodeId>>(mut self, id: T) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn on_click<T: Into<EventId>>(mut self, id: T) -> Self {
        self.on_click = Some(id.into());
        self
    }
}

impl From<Kind> for Box<Node> {
    fn from(value: Kind) -> Self {
        Node::new(value)
    }
}
