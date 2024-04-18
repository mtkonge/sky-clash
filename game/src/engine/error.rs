use sdl2::{video::WindowBuildError, IntegerOrSdlError};

#[derive(Debug, Clone)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_from_T_for_Error {
    ($t:ty) => {
        impl From<$t> for Error {
            fn from(value: $t) -> Self {
                Self(value.to_string())
            }
        }
    };
    ($t:ty, $($ts:ty),+) => {
        impl From<$t> for Error {
            fn from(value: $t) -> Self {
                Self(value.to_string())
            }
        }
        impl_from_T_for_Error!($($ts),+);
    };
}

impl_from_T_for_Error!(String, WindowBuildError, IntegerOrSdlError, &str);
