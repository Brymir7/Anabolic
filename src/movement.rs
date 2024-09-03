use macroquad::math::Vec3;

use crate::config::{MOVE_SPEED, PHYSICS_FRAME_TIME};

pub struct MovementSystem;
impl MovementSystem {
    pub fn update_player(pos: &mut Vec3, vel: &mut Vec3) {
        *pos += *vel * PHYSICS_FRAME_TIME * MOVE_SPEED;
    }
}