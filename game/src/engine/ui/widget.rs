use std::ops::Deref;
use std::rc::Rc;

use crate::engine::{Component, Context, Error, Id};

use super::canvas::Canvas;

use super::units::*;

#[derive(Component, Clone)]
pub struct WidgetRc {
    pub creator_id: Option<Id>,
    pub(super) inner: Rc<dyn Widget>,
}

impl Deref for WidgetRc {
    type Target = dyn Widget;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl WidgetRc {
    pub fn new<T: Widget + 'static>(value: T) -> Self {
        Self {
            creator_id: None,
            inner: Rc::new(value),
        }
    }
    pub fn from_id<T: Widget + 'static>(creator_id: u64, value: T) -> Self {
        Self {
            creator_id: Some(creator_id),
            inner: Rc::new(value),
        }
    }
    pub fn with_id(mut self, id: u64) -> Self {
        self.creator_id = Some(id);
        self
    }
}

impl<T> From<T> for WidgetRc
where
    T: Widget + 'static,
{
    fn from(value: T) -> Self {
        Self {
            creator_id: None,
            inner: Rc::new(value),
        }
    }
}

pub trait Widget {
    fn render(&self, position: Pos, canvas: &mut dyn Canvas) -> Result<(), Error>;
    fn size(&self) -> Size;
}

pub trait WithChildren
where
    Self: Sized,
{
    fn with_child(self, child: WidgetRc) -> Self;
    fn with_children<C: IntoIterator<Item = WidgetRc>>(self, children: C) -> Self {
        children
            .into_iter()
            .fold(self, |parent, child| parent.with_child(child))
    }
}

pub trait WithSize
where
    Self: Sized,
{
    fn with_size<T: Into<Size>>(self, size: T) -> Self;
}

pub trait WithPos
where
    Self: Sized,
{
    fn with_pos<T: Into<Pos>>(self, pos: T) -> Self;
}

pub trait FromChildren
where
    Self: Sized + WithChildren,
{
    fn from_child(child: WidgetRc) -> Self;
    fn from_children<C: IntoIterator<Item = WidgetRc>>(children: C) -> Self;
}

impl<T> FromChildren for T
where
    T: Sized + WithChildren + Default,
{
    fn from_child(child: WidgetRc) -> Self {
        Self::default().with_child(child)
    }

    fn from_children<C: IntoIterator<Item = WidgetRc>>(children: C) -> Self {
        Self::default().with_children(children)
    }
}

pub trait FromSize
where
    Self: Sized + WithSize,
{
    fn from_size<T: Into<Size>>(size: T) -> Self;
}

pub trait FromPos
where
    Self: Sized + WithPos,
{
    fn from_pos<T: Into<Pos>>(pos: T) -> Self;
}

impl<S> FromSize for S
where
    S: Default + Sized + WithSize,
{
    fn from_size<T: Into<Size>>(size: T) -> Self {
        Self::default().with_size(size.into())
    }
}

impl<S> FromPos for S
where
    S: Default + Sized + WithPos,
{
    fn from_pos<T: Into<Pos>>(pos: T) -> Self {
        Self::default().with_pos(pos.into())
    }
}
