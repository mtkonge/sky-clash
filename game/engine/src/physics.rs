use crate::V2;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub pos: V2,
    pub size: V2,
}

impl Rect {
    pub fn new(pos: V2, size: V2) -> Self {
        Self { pos, size }
    }

    pub fn from_f64(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            pos: V2::new(x, y),
            size: V2::new(w, h),
        }
    }

    pub fn top_left(&self) -> V2 {
        self.pos
    }

    pub fn top_right(&self) -> V2 {
        self.pos.add_x(self.size.x)
    }

    pub fn bottom_right(&self) -> V2 {
        self.pos + self.size
    }

    pub fn bottom_left(&self) -> V2 {
        self.pos.add_y(self.size.y)
    }

    pub fn radius(&self) -> f64 {
        self.size.div_comps(2.0).len()
    }

    pub fn radii_distance_to_rect(&self, other: Rect) -> f64 {
        (other.pos - self.pos).len() - (self.radius() + other.radius())
    }

    pub fn side_corners(&self, dir: QuadDirection) -> (V2, V2) {
        use QuadDirection::*;
        let Rect { pos, size } = *self;
        match dir {
            Top => (pos, pos.add_x(size.x)),
            Right => (pos.add_x(size.x), pos + size),
            Bottom => (pos + size, pos.add_y(size.y)),
            Left => (pos.add_y(size.y), pos),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub p0: V2,
    pub p1: V2,
}

impl Line {
    pub fn new(p0: V2, p1: V2) -> Self {
        Self { p0, p1 }
    }

    pub fn direction(&self) -> V2 {
        self.p1 - self.p0
    }

    pub fn is_vertical(&self) -> bool {
        self.p1.x == self.p0.x
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuadDirection {
    Top,
    Right,
    Bottom,
    Left,
}

impl QuadDirection {
    pub fn reverse(&self) -> QuadDirection {
        use QuadDirection::*;
        match self {
            Top => Bottom,
            Right => Left,
            Bottom => Top,
            Left => Right,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OctoDirection {
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl OctoDirection {
    pub fn from_v2(value: V2) -> Option<Self> {
        use std::cmp::Ordering::*;
        use OctoDirection::*;
        match (value.x.total_cmp(&0.0), value.y.total_cmp(&0.0)) {
            (Equal, Greater) => Some(Bottom),
            (Equal, Less) => Some(Top),
            (Greater, Equal) => Some(Right),
            (Less, Equal) => Some(Left),
            (Greater, Greater) => Some(BottomRight),
            (Greater, Less) => Some(TopRight),
            (Less, Greater) => Some(BottomLeft),
            (Less, Less) => Some(TopLeft),
            (Equal, Equal) => None,
        }
    }
    pub fn reverse(&self) -> Self {
        use OctoDirection::*;
        match self {
            Top => Bottom,
            Right => Left,
            Bottom => Top,
            Left => Right,
            TopLeft => BottomRight,
            TopRight => BottomLeft,
            BottomRight => TopLeft,
            BottomLeft => TopRight,
        }
    }

    pub fn clockwise(&self) -> (QuadDirection, QuadDirection) {
        use OctoDirection::{BottomLeft, BottomRight, TopLeft, TopRight};
        use QuadDirection::*;
        match self {
            TopLeft => (Left, Top),
            TopRight => (Top, Right),
            BottomRight => (Right, Bottom),
            BottomLeft => (Bottom, Left),
            _ => unreachable!(),
        }
    }

    pub fn facing(&self, direction: OctoDirection) -> bool {
        use OctoDirection::*;
        match (direction, self) {
            (Top, Top | TopLeft | TopRight)
            | (Right, Right | TopRight | BottomRight)
            | (Bottom, Bottom | BottomLeft | BottomRight) => true,
            (Left, Left | TopLeft | TopRight) => todo!(),
            (Top | Right | Bottom | Left, _) => false,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<OctoDirection> for QuadDirection {
    type Error = ();

    fn try_from(value: OctoDirection) -> Result<Self, Self::Error> {
        use QuadDirection::*;
        match value {
            OctoDirection::Top => Ok(Top),
            OctoDirection::Right => Ok(Right),
            OctoDirection::Bottom => Ok(Bottom),
            OctoDirection::Left => Ok(Left),
            _ => Err(()),
        }
    }
}

impl From<QuadDirection> for OctoDirection {
    fn from(value: QuadDirection) -> Self {
        use OctoDirection::*;
        match value {
            QuadDirection::Top => Top,
            QuadDirection::Right => Right,
            QuadDirection::Bottom => Bottom,
            QuadDirection::Left => Left,
        }
    }
}
