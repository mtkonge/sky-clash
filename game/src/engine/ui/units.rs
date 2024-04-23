use std::ops::{Add, Div, Mul, Sub};

#[derive(Default, Clone, Copy)]
pub struct Size(pub u32, pub u32);
#[derive(Default, Clone, Copy)]
pub struct Pos(pub i32, pub i32);
#[derive(Default, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

macro_rules! derive_math {
    ($struct: ident, $trait: ident, $operand: ident) => {
        impl $trait for $struct {
            type Output = Self;
            fn $operand(self, rhs: Self) -> Self::Output {
                Self(self.0.$operand(rhs.0), self.1.$operand(rhs.1))
            }
        }
    };
}

derive_math!(Size, Add, add);
derive_math!(Size, Mul, mul);
derive_math!(Size, Div, div);
derive_math!(Size, Sub, sub);

derive_math!(Pos, Add, add);
derive_math!(Pos, Mul, mul);
derive_math!(Pos, Div, div);
derive_math!(Pos, Sub, sub);

impl From<(u32, u32)> for Size {
    fn from(value: (u32, u32)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<(i32, i32)> for Pos {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from(value: (u8, u8, u8)) -> Self {
        Self(value.0, value.1, value.2)
    }
}
