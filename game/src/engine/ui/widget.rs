use crate::engine::{Context, Error};

use super::canvas::Canvas;

use super::units::*;

pub struct WidgetWrapper(pub Box<dyn Widget>);

impl<T> From<T> for WidgetWrapper
where
    T: Widget + 'static,
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
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
    fn with_child(self, child: WidgetWrapper) -> Self;
    fn with_children<C: IntoIterator<Item = WidgetWrapper>>(self, children: C) -> Self {
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
    fn from_child(child: WidgetWrapper) -> Self;
    fn from_children<C: IntoIterator<Item = WidgetWrapper>>(children: C) -> Self;
}

impl<T> FromChildren for T
where
    T: Sized + WithChildren + Default,
{
    fn from_child(child: WidgetWrapper) -> Self {
        Self::default().with_child(child)
    }

    fn from_children<C: IntoIterator<Item = WidgetWrapper>>(children: C) -> Self {
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
