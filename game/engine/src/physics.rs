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

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub const F64_TOL: f64 = 0.001;
pub fn eq_tol(lhs: f64, rhs: f64, tol: f64) -> bool {
    (lhs - rhs).abs() <= tol
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

    pub fn move_along(&self, direction: V2, distance: f64) -> Self {
        let t = distance / direction.len();
        *self + direction.extend(t)
    }

    pub fn extend_distance(&self, distance: f64) -> Self {
        let t = (distance + self.len()) / self.len();
        self.extend(t)
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

pub struct Intersection {
    pub pos: V2,
    pub distance_factor: f64,
}

impl Moving<V2> {
    pub fn line_intersect(&self, line: Line) -> Option<V2> {
        let line_direction = line.direction();
        if eq_tol(self.delta_pos.x, 0.0, F64_TOL) && eq_tol(line_direction.x, 0.0, F64_TOL) {
            // parallel, do nothing
            None
        } else if eq_tol(self.delta_pos.x, 0.0, F64_TOL) {
            let x = self.x;
            // y = ax + b
            let line_a = line_direction.y / line_direction.x;
            let line_b = line.p0.y - line_a * line.p0.x;
            let y = line_a * x + line_b;
            Some(V2::new(x, y))
        } else if eq_tol(line_direction.x, 0.0, F64_TOL) {
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
            if eq_tol(delta_pos_a, line_a, F64_TOL) {
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

    /// Will a moving point (self) pass a static point (p), where p is on the same movement path?
    pub fn crosses_point_debug(&self, p: V2) -> bool {
        if eq_tol(self.delta_pos.x, 0.0, F64_TOL) && eq_tol(self.delta_pos.y, 0.0, F64_TOL) {
            // no movement; it will never pass
            return false;
        }
        // draw a line from the target position backwards to the point
        // if the point is on the line, then it intends to pass the point

        // intended_movement = origin->target = (target - origin)
        // point_to_pass = intended_movement * distance_factor + origin
        // distance_factor = (point_to_pass - origin) / intended_movement
        let origin = self.inner;
        let target = origin + self.delta_pos;
        let point_to_pass = p;
        let intended_movement = target - origin;
        let distance_factor = (point_to_pass - origin).len() / intended_movement.len();

        if distance_factor < -0.01 {
            // has not yet reached point
            dbg!(origin, intended_movement, point_to_pass, distance_factor);
            return false;
        }
        if distance_factor > 1.0 {
            // has already passed point
            println!("passed T~T");
            return false;
        }
        true
    }

    /// Will a moving point (self) pass a static point (p), where p is on the same movement path?
    pub fn crosses_point(&self, p: V2) -> bool {
        if eq_tol(self.delta_pos.x, 0.0, F64_TOL) && eq_tol(self.delta_pos.y, 0.0, F64_TOL) {
            // no movement; it will never pass
            return false;
        }
        // draw a line from the target position backwards to the point
        // if the point is on the line, then it intends to pass the point

        // intended_movement = origin->target = (target - origin)
        // point_to_pass = intended_movement * distance_factor + origin
        // distance_factor = (point_to_pass - origin) / intended_movement
        let origin = self.inner;
        let target = origin + self.delta_pos;
        let point_to_pass = p;
        let intended_movement = target - origin;
        let distance_factor = (point_to_pass - origin).len() / intended_movement.len();

        if distance_factor < 0.0 {
            // has not yet reached point
            return false;
        }
        if distance_factor > 1.0 {
            // has already passed point
            return false;
        }
        true
    }

    pub fn distance_factor_to_point(&self, p: V2) -> f64 {
        (p - **self).len() / self.delta_pos.len()
    }

    /// Calculates intersection between point and line if exists.
    /// Returns position and distance factor.
    /// The close intersection is to point, the closer factor is to zero,
    /// factor is zero, when intersection is at point + delta_pos.
    pub fn line_segment_intersect(&self, line: Line) -> Option<Intersection> {
        // TODO: uhhhh keep debugging idk
        let debug = false && line.p0.x == 306.0 && line.p0.y == 162.0 && self.inner.x < 600.0;
        if debug {
            println!("started new");
            if self.inner.x < 306.0 {
                // panic!();
            }
        }
        if eq_tol(self.delta_pos.len(), 0.0, F64_TOL) {
            // no movement, no collision
            return None;
        }
        let intersection = self.line_intersect(line)?;
        if !line.point_within_segment(intersection) {
            return None;
        }

        let cwosses = if debug {
            self.crosses_point_debug(intersection)
        } else {
            self.crosses_point(intersection)
        };

        if !cwosses {
            if debug {
                println!("doth not cwoss point");
                dbg!(intersection, self.inner.x, self.delta_pos.x);
            }
            return None;
        }
        let distance_factor = self.distance_factor_to_point(intersection);
        Some(Intersection {
            pos: intersection,
            distance_factor,
        })
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
        eq_tol(self.p1.x, self.p0.x, F64_TOL)
    }

    pub fn point_on_line(&self, p: V2) -> bool {
        let r = self.p1 - self.p0;
        if eq_tol(r.x, 0.0, F64_TOL) && eq_tol(self.p0.x, p.x, F64_TOL) {
            true
        } else if eq_tol(r.y, 0.0, F64_TOL) && eq_tol(self.p0.y, p.y, F64_TOL) {
            true
        } else {
            let t_x = (p.x - self.p0.x) / r.x;
            let t_y = (p.y - self.p0.y) / r.y;
            eq_tol(t_x, t_y, F64_TOL)
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
