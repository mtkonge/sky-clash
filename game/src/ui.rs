use super::engine::{self, Component, System};
use crate::{engine::Id, hero_creator, query};
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    rc::{Rc, Weak},
};

pub enum Size {
    Min,
    Max,
    Fix(i32),
}

pub enum Align {
    Begin,
    Center,
    End,
}

pub enum Kind {
    Vertical(Vec<Rc<RefCell<Node>>>),
    Horizontal(Vec<Rc<RefCell<Node>>>),
    Title(String),
    Button(String),
}

#[derive(Clone)]
struct Layout {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

pub struct Node {
    kind: Kind,
    parent: Option<Weak<Node>>,
    layout: Option<Layout>,
    pub id: Option<String>,
    pub text_color: Option<(u8, u8, u8)>,
    pub background_color: Option<(u8, u8, u8)>,
    pub h_align: Option<Align>,
    pub v_align: Option<Align>,
    pub width: Size,
    pub height: Size,
    pub font: Option<String>,
    pub font_size: Option<i32>,
}

macro_rules! builder_function_option {
    ($method_name:ident, $field_name:ident, $type:ty) => {
        pub fn $method_name(mut self, $field_name: $type) -> Self {
            Self {
                $field_name: Some($field_name),
                ..self
            }
        }
    };
}

macro_rules! builder_function {
    ($method_name:ident, $field_name:ident, $type:ty, $expr:expr) => {
        pub fn $method_name(mut self, $field_name: $type) -> Self {
            Self {
                $field_name: $expr,
                ..self
            }
        }
    };
}

const OVERFLOW_AVOIDER: i32 = 1;

impl Node {
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            parent: None,
            layout: None,
            id: None,
            text_color: None,
            background_color: None,
            h_align: None,
            v_align: None,
            width: Size::Min,
            height: Size::Min,
            font: None,
            font_size: None,
        }
    }

    builder_function_option!(with_id, id, String);
    builder_function_option!(with_text_color, text_color, (u8, u8, u8));
    builder_function_option!(with_background_color, background_color, (u8, u8, u8));
    builder_function_option!(with_h_align, h_align, Align);
    builder_function_option!(with_v_align, v_align, Align);
    builder_function!(with_width, width, Size, width);
    builder_function!(with_height, height, Size, height);
    builder_function_option!(with_font, font, String);
    builder_function_option!(with_font_size, font_size, i32);

    pub fn as_head(mut self) -> Self {
        self.with_text_color((255, 255, 255))
            .with_background_color((0, 0, 0))
            .with_h_align(Align::Begin)
            .with_v_align(Align::Begin)
            .with_width(Size::Max)
            .with_height(Size::Max)
            .with_font("textures/ttf/OpenSans.ttf".to_string())
            .with_font_size(16)
    }

    pub fn build(mut self) -> Rc<RefCell<Self>> {
        self.eliminate_those_pesky_fucking_orphans();
        Rc::new(RefCell::new(self))
    }

    fn rehome_rec(&mut self, parent: Option<Weak<Node>>) {
        self.parent = parent;
        match self.kind {
            Kind::Vertical(children) | Kind::Horizontal(children) => {
                for mut child in children {
                    child
                        .try_borrow_mut()
                        .unwrap()
                        .rehome_rec(Some(Weak::new()))
                }
            }
            Kind::Title(_) | Kind::Button(_) => {}
        }
    }

    pub fn eliminate_those_pesky_fucking_orphans(&mut self) {
        self.rehome_rec(None)
    }

    fn get_prop<T, F: Fn(&Self) -> Option<T>>(&self, f: F) -> Option<T> {
        if let Some(v) = f(self) {
            return Some(v);
        }
        if let Some(parent) = self.parent {
            if let Some(v) = f(parent.upgrade()?.as_ref()) {
                return Some(v);
            }
        }
        None
    }

    pub fn layout(
        &mut self,
        ctx: &mut engine::Context,
        pos: (i32, i32),
        outer_size: (i32, i32),
    ) -> (i32, i32) {
        match self.kind {
            Kind::Vertical(children) => {
                let mut child_pos = pos;
                let mut size = (0, 0);
                for child in children {
                    let child_size = child
                        .try_borrow_mut()
                        .unwrap()
                        .layout(ctx, child_pos, outer_size);
                    size.0 += child_size.0;
                    size.1 += child_size.1;
                    child_pos.1 += size.1;
                }
                if size.0 > outer_size.0 || size.1 > outer_size.1 {
                    panic!("overflow, idk what to do >~<");
                }
                let v_align = self.get_prop(|n| n.v_align);
                let pos_y = match v_align {
                    Some(Align::Begin) => pos.1,
                    Some(Align::Center) => {
                        pos.1
                            + match self.height {
                                Size::Min => size.1,
                                Size::Max => outer_size.1 - size.1 - (outer_size.1 - size.1) / 2,
                                Size::Fix(width) => width,
                            }
                    }
                    Some(Align::End) => {
                        pos.1
                            + match self.height {
                                Size::Min => 0,
                                Size::Max => outer_size.1 - size.1,
                                Size::Fix(height) => {
                                    assert!(size.1 <= height, "overflow >~<");
                                    height - size.1
                                }
                            }
                    }
                    None => unreachable!(),
                };
                let h_align = self.get_prop(|n| n.h_align);
                let width = match h_align {
                    Some(Align::Begin) => match self.width {
                        Size::Min => size.0,
                        Size::Max => outer_size.0,
                        Size::Fix(width) => width,
                    },
                    Some(Align::Center) => {
                        let mut child_pos = (pos.0, pos_y);
                        let mut size = (0, 0);
                        for child in children {
                            let layout = child.try_borrow_mut().unwrap().layout.unwrap().clone();
                            let child_size = child.try_borrow_mut().unwrap().layout(
                                ctx,
                                (
                                    outer_size.0 - layout.width - (outer_size.0 - layout.width) / 2,
                                    child_pos.0,
                                ),
                                outer_size,
                            );
                            size.1 += child_size.1;
                            child_pos.1 += size.1;
                        }
                        outer_size.0
                    }
                    Some(Align::End) => {
                        let mut child_pos = (pos.0, pos_y);
                        let mut size = (0, 0);
                        for child in children {
                            let layout = child.try_borrow_mut().unwrap().layout.unwrap().clone();
                            let child_size = child.try_borrow_mut().unwrap().layout(
                                ctx,
                                (outer_size.0 - layout.width, child_pos.0),
                                outer_size,
                            );
                            size.1 += child_size.1;
                            child_pos.1 += size.1;
                        }
                        outer_size.0
                    }
                    None => unreachable!(),
                };
                (width, outer_size.0)
            }
            Kind::Horizontal(_) => todo!(),
            Kind::Title(text) => {
                let font = ctx
                    .load_font(
                        self.get_prop(|n| n.font).unwrap(),
                        self.get_prop(|n| n.font_size).unwrap() as u16,
                    )
                    .unwrap();
                let text_size = ctx.text_size(font, &text).unwrap();
                assert!(text_size.0 as i32 <= outer_size.0, "overflow >~<");
                let width = match self.width {
                    Size::Min => text_size.0 as i32,
                    Size::Max => outer_size.0,
                    Size::Fix(w) => {
                        assert!(text_size.0 as i32 <= w, "overflow >~<");
                        assert!(w as i32 <= outer_size.0, "overflow >~<");
                        w
                    }
                };
                assert!(text_size.1 as i32 <= outer_size.1, "overflow >~<");
                let height = match self.height {
                    Size::Min => text_size.1 as i32,
                    Size::Max => outer_size.1,
                    Size::Fix(h) => {
                        assert!(text_size.1 as i32 <= h, "overflow >~<");
                        assert!(h as i32 <= outer_size.1, "overflow >~<");
                        h
                    }
                };
                (width, height)
            }
            Kind::Button(text) => {
                let font = ctx
                    .load_font(
                        self.get_prop(|n| n.font).unwrap(),
                        self.get_prop(|n| n.font_size).unwrap() as u16,
                    )
                    .unwrap();
                let text_size = ctx.text_size(font, &text).unwrap();
                assert!(text_size.0 as i32 <= outer_size.0, "overflow >~<");
                let width = match self.width {
                    Size::Min => text_size.0 as i32,
                    Size::Max => outer_size.0,
                    Size::Fix(w) => {
                        assert!(text_size.0 as i32 <= w, "overflow >~<");
                        assert!(w as i32 <= outer_size.0, "overflow >~<");
                        w
                    }
                };
                assert!(text_size.1 as i32 <= outer_size.1, "overflow >~<");
                let height = match self.height {
                    Size::Min => text_size.1 as i32,
                    Size::Max => outer_size.1,
                    Size::Fix(h) => {
                        assert!(text_size.1 as i32 <= h, "overflow >~<");
                        assert!(h as i32 <= outer_size.1, "overflow >~<");
                        h
                    }
                };
                (width, height)
            }
        }
    }

    pub fn draw(&self, ctx: &mut engine::Context) {
        match self.kind {
            Kind::Vertical(children) => {
                for child in children {
                    child.try_borrow_mut().unwrap().draw(ctx);
                }
            }
            Kind::Horizontal(children) => {
                for child in children {
                    child.try_borrow_mut().unwrap().draw(ctx);
                }
            }
            Kind::Title(text) => {}
            Kind::Button(_) => todo!(),
        }
    }
}

#[derive(Component)]
pub struct UIComponent {
    pub system_id: Id,
    pub dom: Node,
    pub screen_size: (i32, i32),
}

pub struct UISystem(pub Id);

impl System for UISystem {
    fn on_update(&self, ctx: &mut engine::Context, _delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, UIComponent) {
            let component = ctx.entity_component::<UIComponent>(id);
            // component.dom.draw(ctx, (0, 0), component.screen_size);
        }
        Ok(())
    }
}
