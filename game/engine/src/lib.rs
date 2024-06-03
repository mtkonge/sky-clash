#![allow(unused_imports)]

mod component;
mod context;
mod entity;
mod error;
mod font;
mod game;
mod id;
mod query_runner;
mod system;
mod text;
mod texture;
mod v2;

pub mod collision;
pub mod physics;
pub mod rigid_body;
pub mod shared_ptr;
pub mod ui;

pub use self::{
    collision::CollisionSystem, collision::SolidCollider, component::Component,
    context::ComponentQuery, context::Context, context::DrawTextureOpts, error::Error, game::Game,
    id::Id, query_runner::QueryRunner, shared_ptr::SharedPtr, system::System, text::Text,
    texture::Texture, v2::clamp, v2::max, v2::min, v2::V2,
};
pub use component_macro::Component;
pub use sdl2::controller::Button as ControllerButton;
pub use sdl2::keyboard::Keycode;
pub use sdl2::mouse::MouseButton;
