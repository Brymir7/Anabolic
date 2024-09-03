use macroquad::math::Vec3;

use crate::{
    config::{GRAVITY, MOVE_SPEED, PHYSICS_FRAME_TIME},
    types::{ChunkVec3, EntityType},
};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update_player(pos: &mut ChunkVec3, vel: &mut Vec3, chunk: &Vec<Vec<Vec<EntityType>>>) {
        vel.y += GRAVITY * PHYSICS_FRAME_TIME;
        
        // Y-axis movement and collision
        let mut new_pos = ChunkVec3(pos.0 + Vec3::new(0.0, vel.y, 0.0) * PHYSICS_FRAME_TIME * MOVE_SPEED);
        if Self::check_collision(&new_pos, chunk) {
            vel.y = 0.0;
            new_pos = *pos;
        }
        
        // X-axis movement and collision
        new_pos = ChunkVec3(new_pos.0 + Vec3::new(vel.x, 0.0, 0.0) * PHYSICS_FRAME_TIME * MOVE_SPEED);
        if Self::check_collision(&new_pos, chunk) {
            vel.x = 0.0;
            new_pos = ChunkVec3(pos.0 + Vec3::new(0.0, new_pos.0.y - pos.0.y, 0.0));
        }
        
        // Z-axis movement and collision
        new_pos = ChunkVec3(new_pos.0 + Vec3::new(0.0, 0.0, vel.z) * PHYSICS_FRAME_TIME * MOVE_SPEED);
        if Self::check_collision(&new_pos, chunk) {
            vel.z = 0.0;
            new_pos = ChunkVec3(pos.0 + Vec3::new(new_pos.0.x - pos.0.x, new_pos.0.y - pos.0.y, 0.0));
        }
        
        *pos = new_pos;
    }

    fn check_collision(pos: &ChunkVec3, chunk: &Vec<Vec<Vec<EntityType>>>) -> bool {
        let chunk_pos = pos.to_chunk();
        
        // Check if the position is within the chunk bounds
        if chunk_pos.x < 0 || chunk_pos.x as usize >= chunk.len() ||
           chunk_pos.z < 0 || chunk_pos.z as usize >= chunk[0].len() ||
           chunk_pos.y < 0 || chunk_pos.y as usize >= chunk[0][0].len() {
            return true; // Collision with chunk boundary
        }

        // Check if the entity at this position is solid
        match chunk[chunk_pos.x as usize][chunk_pos.z as usize][chunk_pos.y as usize] {
            EntityType::SolidBlock => true,
            _ => false,
        }
    }
}
