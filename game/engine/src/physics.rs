use std::ops::Deref;

pub fn min<T: PartialOrd>(lhs: T, rhs: T) -> T {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

pub fn max<T: PartialOrd>(lhs: T, rhs: T) -> T {
    if lhs > rhs {
        lhs
    } else {
        rhs
    }
}

pub fn clamp(value: f64, max: f64) -> f64 {
    if value > max {
        max
    } else {
        value
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Moving<T> {
    inner: T,
    pub delta_pos: V2,
}

impl<T> Moving<T> {
    pub fn new(inner: T, delta_pos: V2) -> Self {
        Self { inner, delta_pos }
    }
}

impl<T> Deref for Moving<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait Movable
where
    Self: Sized,
{
    fn moving(self, delta_pos: V2) -> Moving<Self>;
}

impl<T> Movable for T {
    fn moving(self, delta_pos: V2) -> Moving<Self> {
        Moving::new(self, delta_pos)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

impl V2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn extend(&self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }

    pub fn div_comps(&self, rhs: f64) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }

    pub fn min_comp(&self) -> f64 {
        std::cmp::min_by(self.x, self.y, f64::total_cmp)
    }

    pub fn max_comp(&self) -> f64 {
        std::cmp::max_by(self.x, self.y, f64::total_cmp)
    }

    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn reverse(&self) -> Self {
        Self::new(-self.x, -self.y)
    }

    pub fn add_x(&self, rhs: f64) -> Self {
        Self::new(self.x + rhs, self.y)
    }

    pub fn add_y(&self, rhs: f64) -> Self {
        Self::new(self.x, self.y + rhs)
    }
}

impl std::ops::Add for V2 {
    type Output = V2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for V2 {
    type Output = V2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::AddAssign for V2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<(f64, f64)> for V2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl Default for V2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Moving<V2> {
    pub fn line_intersect(&self, line: Line) -> Option<V2> {
        let line_direction = line.direction();
        if self.delta_pos.x == 0.0 && line_direction.x == 0.0 {
            // parallel, do nothing
            None
        } else if self.delta_pos.x == 0.0 {
            let x = self.x;
            // y = ax + b
            let line_a = line_direction.y / line_direction.x;
            let line_b = line.p0.y - line_a * line.p0.x;
            let y = line_a * x + line_b;
            Some(V2::new(x, y))
        } else if line_direction.x == 0.0 {
            let x = line.p0.x;
            // y = ax + b
            let delta_pos_a = self.delta_pos.y / self.delta_pos.x;
            let delta_pos_b = self.y - delta_pos_a * self.x;
            let y = delta_pos_a * x + delta_pos_b;
            Some(V2::new(x, y))
        } else {
            // y = ax + b
            let delta_pos_a = self.delta_pos.y / self.delta_pos.x;
            let line_a = line_direction.y / line_direction.x;
            if delta_pos_a == line_a {
                // parallel: either none or continous intersection
                return None;
            }
            let delta_pos_b = self.y - delta_pos_a * self.x;
            let line_b = line.p0.y - line_a * line.p0.x;
            let x = (line_b - delta_pos_b) / (delta_pos_a - line_a);
            let y = delta_pos_a * x + delta_pos_b;
            Some(V2::new(x, y))
        }
    }

    pub fn crosses_point(&self, p: V2) -> bool {
        let p = p;
        let pos_s = if self.delta_pos.x == 0.0 {
            (p.y - self.y) / self.delta_pos.y
        } else {
            (p.x - self.x) / self.delta_pos.x
        };
        let delta_pos_s = if self.delta_pos.x == 0.0 {
            (p.y - (self.y + self.delta_pos.y)) / self.delta_pos.y
        } else {
            (p.x - (self.x + self.delta_pos.x)) / self.delta_pos.x
        };
        if pos_s * delta_pos_s > 0.0 {
            // wrong side
            return false;
        }
        if delta_pos_s >= 0.0 {
            // out of range
            return false;
        }
        true
    }

    pub fn distance_factor_to_point(&self, p: V2) -> f64 {
        // intersection = pos + delta_pos * score
        // (intersection - pos) / delta_pos = score
        if self.delta_pos.x != 0.0 {
            (p.x - self.x) / self.delta_pos.x
        } else if self.delta_pos.y != 0.0 {
            (p.y - self.y) / self.delta_pos.y
        } else {
            unreachable!("already verified delta_pos != (0, 0)");
        }
    }

    /// Calculates intersection between point and line if exists.
    /// Returns position and distance factor.
    /// The close intersection is to point, the closer factor is to zero,
    /// factor is zero, when intersection is at point + delta_pos.
    pub fn line_segment_intersect(&self, line: Line) -> Option<(V2, f64)> {
        if self.delta_pos.len() == 0.0 {
            // no movement, no collision
            return None;
        }
        let intersection = self.line_intersect(line)?;
        if !line.point_within_segment(intersection) {
            return None;
        }
        if !self.crosses_point(intersection) {
            return None;
        }
        let score = self.distance_factor_to_point(intersection);
        Some((intersection, score))
    }
}

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

impl Moving<Rect> {
    pub fn rect_within_reach(&self, other: Rect) -> bool {
        let radii = self.radius() + self.delta_pos.len() + other.radius();
        let length_between = self.radii_distance_to_rect(other);
        radii >= length_between
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

    pub fn point_on_line(&self, p: V2) -> bool {
        let r = self.p1 - self.p0;
        if r.x == 0.0 && self.p0.x == p.x {
            true
        } else if r.y == 0.0 && self.p0.y == p.y {
            true
        } else {
            let t_x = (p.x - self.p0.x) / r.x;
            let t_y = (p.y - self.p0.y) / r.y;
            t_x == t_y
        }
    }

    pub fn point_within_segment(&self, p: V2) -> bool {
        if !self.point_on_line(p) {
            return false;
        }
        let t = if self.is_vertical() {
            (p.y - self.p0.y) / (self.p1.y - self.p0.y)
        } else {
            (p.x - self.p0.x) / (self.p1.x - self.p0.x)
        };
        (0.0..=1.0).contains(&t)
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

#[test]
fn test_rects_within_reach() {
    assert!(Rect::from_f64(0.0, 0.0, 10.0, 0.0)
        .moving(V2::new(10.0, 10.0))
        .rect_within_reach(Rect::from_f64(15.0, 0.0, 10.0, 10.0)));
    assert!(!Rect::from_f64(0.0, 0.0, 10.0, 0.0)
        .moving(V2::new(10.0, 10.0))
        .rect_within_reach(Rect::from_f64(40.0, 0.0, 10.0, 10.0)));
}

#[test]
fn test_point_vec_line_segment_intersect() {
    macro_rules! named {
        ($name: ident) => {
            (stringify!($name), $name)
        };
    }
    let check_a = {
        let edge_a = (V2::new(10.0, 10.0), V2::new(40.0, 10.0));
        let line_a = (V2::new(20.0, 0.0), V2::new(10.0, 20.0));
        let line_b = (V2::new(25.0, 0.0), V2::new(0.0, 25.0));
        let line_c = (V2::new(30.0, 0.0), V2::new(-10.0, 20.0));
        let intersection = V2::new(25.0, 10.0);

        [named!(line_a), named!(line_b), named!(line_c)]
            .into_iter()
            .map(|line| (line, named!(edge_a), intersection))
            .collect::<Vec<_>>()
    };
    let check_b = {
        let edge_b = (V2::new(40.0, 40.0), V2::new(40.0, 10.0));
        let line_d = (V2::new(50.0, 20.0), V2::new(-20.0, 10.0));
        let line_e = (V2::new(50.0, 25.0), V2::new(-25.0, 0.0));
        let line_f = (V2::new(50.0, 30.0), V2::new(-20.0, -10.0));
        let intersection = V2::new(40.0, 25.0);

        [named!(line_d), named!(line_e), named!(line_f)]
            .into_iter()
            .map(|line| (line, named!(edge_b), intersection))
            .collect::<Vec<_>>()
    };
    let check_c = {
        let edge_c = (V2::new(40.0, 40.0), V2::new(10.0, 40.0));
        let line_i = (V2::new(20.0, 50.0), V2::new(10.0, -20.0));
        let line_h = (V2::new(25.0, 50.0), V2::new(0.0, -25.0));
        let line_g = (V2::new(30.0, 50.0), V2::new(-10.0, -20.0));
        let intersection = V2::new(25.0, 40.0);

        [named!(line_i), named!(line_h), named!(line_g)]
            .into_iter()
            .map(|line| (line, named!(edge_c), intersection))
            .collect::<Vec<_>>()
    };
    let check_d = {
        let edge_d = (V2::new(10.0, 10.0), V2::new(10.0, 40.0));
        let line_d = (V2::new(0.0, 20.0), V2::new(20.0, 10.0));
        let line_e = (V2::new(0.0, 25.0), V2::new(25.0, 0.0));
        let line_f = (V2::new(0.0, 30.0), V2::new(20.0, -10.0));
        let intersection = V2::new(10.0, 25.0);

        [named!(line_d), named!(line_e), named!(line_f)]
            .into_iter()
            .map(|line| (line, named!(edge_d), intersection))
            .collect::<Vec<_>>()
    };
    [check_a, check_b, check_c, check_d]
        .into_iter()
        .flatten()
        .for_each(
            |(
                (line_name, (pos, delta_pos)),
                (edge_name, (edge_p0, edge_p1)),
                expected_intersection,
            )| {
                let intersection =
                    pos.moving(delta_pos).line_segment_intersect(Line::new(edge_p0, edge_p1))
                        .map(|(intersection, _score)| intersection);

                assert!(
                    intersection.is_some(),
                    "expected line {line_name} to intersect with edge {edge_name}, got None"
                );

                let intersection = intersection.expect("we asserted it to be Some");
                assert_eq!(intersection, expected_intersection, "expected line {line_name} to intersect with edge {edge_name} at {expected_intersection:?}, got {intersection:?}")
            },
        );
}
