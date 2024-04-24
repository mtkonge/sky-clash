use std::ops::Deref;
use std::rc::Rc;

use crate::engine::{Component, Context, Error, Id};

use super::canvas::Canvas;

use super::units::*;

#[derive(Clone)]
pub struct Root {
    pub creator_id: Id,
    inner: WidgetPointer,
}

impl Deref for Root {
    type Target = dyn Widget;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl Root {
    pub fn new<T>(creator_id: Id, inner: T) -> Self
    where
        T: Into<WidgetPointer>,
    {
        Self {
            creator_id,
            inner: inner.into(),
        }
    }
}

impl Component for Root {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct WidgetPointer(Rc<dyn Widget>);

impl<T> From<T> for WidgetPointer
where
    T: Widget + 'static,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Deref for WidgetPointer {
    type Target = dyn Widget;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl WidgetPointer {
    pub fn new<T: Widget + 'static>(value: T) -> Self {
        Self(Rc::new(value))
    }
}

pub trait Widget {
    fn render(&self, offset: Offset, canvas: &mut dyn Canvas) -> Result<(), Error>;
    fn size(&self) -> Size;
    fn resolve_events(&self, _event_pool: Rc<std::sync::Mutex<u32>>) {}
}

pub trait WithChildren
where
    Self: Sized,
{
    fn with_child(self, child: WidgetPointer) -> Self;
    fn with_children<C: IntoIterator<Item = WidgetPointer>>(self, children: C) -> Self {
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
    fn with_pos<T: Into<Offset>>(self, pos: T) -> Self;
}

pub trait FromChildren
where
    Self: Sized + WithChildren,
{
    fn from_child(child: WidgetPointer) -> Self;
    fn from_children<C: IntoIterator<Item = WidgetPointer>>(children: C) -> Self;
}

impl<T> FromChildren for T
where
    T: Sized + WithChildren + Default,
{
    fn from_child(child: WidgetPointer) -> Self {
        Self::default().with_child(child)
    }

    fn from_children<C: IntoIterator<Item = WidgetPointer>>(children: C) -> Self {
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
    fn from_pos<T: Into<Offset>>(pos: T) -> Self;
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
    fn from_pos<T: Into<Offset>>(pos: T) -> Self {
        Self::default().with_pos(pos.into())
    }
}
