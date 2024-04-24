use crate::engine::{
    ui::{
        canvas::Canvas,
        units::{Offset, Size},
        widget::{Widget, WidgetPointer, WithChildren},
    },
    Error, Id,
};

use super::shape::Rect;

#[derive(Default)]
pub struct HorizontallyCentered {
    pos: Offset,
    children: Vec<WidgetPointer>,
}

impl Widget for HorizontallyCentered {
    fn render(&self, pos: Offset, canvas: &mut dyn Canvas) -> Result<(), Error> {
        let pos = pos + self.pos;
        let size = self.size();
        let mut y = pos.1;
        for child in &self.children {
            let child_size = child.size();
            let x = ((size.0 - child_size.0) / 2) as i32;
            child.render(Offset(x, y), canvas)?;
            y += child_size.1 as i32;
        }
        Ok(())
    }

    fn size(&self) -> Size {
        self.children.iter().fold(Size(0, 0), |acc, curr| {
            Size(std::cmp::max(acc.0, curr.size().0), acc.1 + curr.size().1)
        })
    }

    fn child_pointers(&self) -> Option<Vec<WidgetPointer>> {
        Some(self.children.clone())
    }
}

impl WithChildren for HorizontallyCentered {
    fn with_child(mut self, child: WidgetPointer) -> Self {
        self.children.push(child);
        self
    }
}

#[derive(Default)]
pub struct VerticallyCentered {
    pos: Offset,
    children: Vec<WidgetPointer>,
}

impl Widget for VerticallyCentered {
    fn render(&self, pos: Offset, canvas: &mut dyn Canvas) -> Result<(), Error> {
        let pos = pos + self.pos;
        let size = self.size();
        let mut x = pos.0;
        for child in &self.children {
            let child_size = child.size();
            let y = ((size.1 - child_size.1) / 2) as i32;
            child.render(Offset(x, y), canvas)?;
            x += child_size.0 as i32;
        }
        Ok(())
    }

    fn size(&self) -> Size {
        self.children.iter().fold(Size(0, 0), |acc, curr| {
            Size(acc.0 + curr.size().0, std::cmp::max(acc.1, curr.size().1))
        })
    }

    fn child_pointers(&self) -> Option<Vec<WidgetPointer>> {
        Some(self.children.clone())
    }
}

impl WithChildren for VerticallyCentered {
    fn with_child(mut self, child: WidgetPointer) -> Self {
        self.children.push(child);
        self
    }
}
