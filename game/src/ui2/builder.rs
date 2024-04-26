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
    Rect,
    Vert(Vec<Box<Node>>),
    Hori(Vec<Box<Node>>),
    Text(String),
}

pub mod constructors {
    #![allow(non_snake_case)]
    use super::*;
    pub fn Rect() -> Box<Node> {
        Kind::Rect.into()
    }
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
    id: Option<u64>,
    width: Option<i32>,
    height: Option<i32>,
    on_click: Option<u64>,
    background_color: Option<(u8, u8, u8)>,
    color: Option<(u8, u8, u8)>,
    border_thickness: Option<i32>,
    border_color: Option<(u8, u8, u8)>,
    padding: Option<i32>,
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
        })
    }

    pub fn build(
        &mut self,
        nodes: &mut Vec<(NodeId, super::Node)>,
        id_counter: &mut u64,
        mut derived_props: DerivedProps,
    ) -> super::NodeId {
        if self.color.is_none() {
            self.color = derived_props.color;
        } else if derived_props.color.is_none() {
            derived_props.color = self.color;
        }

        let id = super::NodeId(*id_counter);
        *id_counter += 1;
        let kind = match &mut self.kind {
            Kind::Rect => super::Kind::Rect,
            Kind::Vert(ref mut children) => {
                let mut children_ids = Vec::new();
                for mut child in children.drain(..) {
                    children_ids.push(child.build(nodes, id_counter, derived_props.clone()));
                }
                super::Kind::Vert(children_ids)
            }
            Kind::Hori(ref mut children) => {
                let mut children_ids = Vec::new();
                for mut child in children.drain(..) {
                    children_ids.push(child.build(nodes, id_counter, derived_props.clone()));
                }
                super::Kind::Hori(children_ids)
            }
            Kind::Text(v) => super::Kind::Text {
                text: v.clone(),
                font: PathBuf::from("textures/ttf/OpenSans.ttf"),
                size: 15,
            },
        };
        nodes.push((
            id,
            super::Node {
                kind,
                id: self.id,
                width: self.width,
                height: self.height,
                on_click: self.on_click,
                background_color: self.background_color,
                color: self.color,
                border_color: self.border_color,
                border_thickness: self.border_thickness,
                padding: self.padding,
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
    make_with_function!(with_id, id, u64);
    make_with_function!(with_width, width, i32);
    make_with_function!(with_height, height, i32);
    make_with_function!(with_background_color, background_color, (u8, u8, u8));
    make_with_function!(with_color, color, (u8, u8, u8));
    make_with_function!(with_border_color, border_color, (u8, u8, u8));
    make_with_function!(with_border_thickness, border_thickness, i32);
    make_with_function!(with_padding, padding, i32);
}

impl From<Kind> for Box<Node> {
    fn from(value: Kind) -> Self {
        Node::new(value)
    }
}
