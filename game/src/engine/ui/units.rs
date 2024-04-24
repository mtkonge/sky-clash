use std::ops::{Add, Div, Mul, Sub};

#[derive(Default, Clone, Copy)]
pub struct Size(pub u32, pub u32);
#[derive(Default, Clone, Copy)]
pub struct Offset(pub i32, pub i32);
#[derive(Default, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

macro_rules! derive_math {
    ($struct: ident, $trait: ident, $operand: ident, $internal_type: ident) => {
        impl $trait<Self> for $struct {
            type Output = Self;
            fn $operand(self, rhs: Self) -> Self::Output {
                Self(self.0.$operand(rhs.0), self.1.$operand(rhs.1))
            }
        }

        impl $trait<$internal_type> for $struct {
            type Output = Self;
            fn $operand(self, rhs: $internal_type) -> Self::Output {
                Self(self.0.$operand(rhs), self.1.$operand(rhs))
            }
        }
    };
}

derive_math!(Size, Add, add, u32);
derive_math!(Size, Mul, mul, u32);
derive_math!(Size, Div, div, u32);
derive_math!(Size, Sub, sub, u32);

derive_math!(Offset, Add, add, i32);
derive_math!(Offset, Mul, mul, i32);
derive_math!(Offset, Div, div, i32);
derive_math!(Offset, Sub, sub, i32);

impl From<(u32, u32)> for Size {
    fn from(value: (u32, u32)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<(i32, i32)> for Offset {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from(value: (u8, u8, u8)) -> Self {
        Self(value.0, value.1, value.2)
    }
}
