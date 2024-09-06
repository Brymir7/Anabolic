pub mod config;
pub mod types;
pub mod type_impl;
pub use macroquad::{
    color::{self, GREEN, RED},
    math::{self, vec3, Vec3},
    models::{self, draw_cube_wires},
    prelude::*,
};
pub use once_cell::sync::Lazy;
