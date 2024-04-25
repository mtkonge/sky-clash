use std::cell::{Cell, Ref, RefCell};
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
    type Target = WidgetPointer;

    fn deref(&self) -> &Self::Target {
        &self.inner
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

    pub fn widget_with_id(self, id: Id) -> (WidgetPointer, Option<WidgetPointer>) {
        (self.inner.clone(), self.inner.widget_with_id(id))
    }
}

impl Component for Root {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct WidgetPointer(Option<Id>, Rc<dyn Widget>);

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
        self.1.deref()
    }
}

impl WidgetPointer {
    pub fn new<T: Widget + 'static>(value: T) -> Self {
        Self(None, Rc::new(value))
    }
    pub fn set<T: Widget + 'static>(&self, value: T) {
        let raw = Rc::<(dyn Widget + 'static)>::as_ptr(&self.1) as *mut T;
        unsafe { *raw = value }
    }
    pub fn with_id(mut self, id: Id) -> Self {
        self.0 = Some(id);
        self
    }
    pub fn widget_with_id(self, id: Id) -> Option<WidgetPointer> {
        
        if self.0.is_some_and(|v| v == id) {
            Some(self.clone())
        } else {
            match self.child_pointers() {
                Some(ptrs) => ptrs.into_iter().find_map(|w| w.widget_with_id(id)),
                None => None,
            }
        }
    }
}

pub trait Widget {
    fn render(&self, offset: Offset, canvas: &mut dyn Canvas) -> Result<(), Error>;
    fn size(&self) -> Size;
    fn resolve_events(&self, _event_pool: Rc<std::sync::Mutex<u32>>) {
        todo!()
    }
    fn child_pointers(&self) -> Option<Vec<WidgetPointer>> {
        None
    }
}

pub trait WidgetWithId: Widget
where
    Self: Sized + 'static,
{
    fn with_id(self, id: Id) -> WidgetPointer {
        WidgetPointer::new(self).with_id(id)
    }
}

impl<T> WidgetWithId for T where T: Widget + Sized + 'static {}

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
