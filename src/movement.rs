use macroquad::{ color::RED, math::Vec3, models::draw_cube_wires };

use crate::{
    config::{ CHUNK_SIZE, GRAVITY, MOVE_SPEED, PHYSICS_FRAME_TIME },
    types::{ ChunkVec3, EntityType },
};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update_player(
        pos: &mut ChunkVec3,
        vel: &mut Vec3,
        chunk: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) {
        vel.y += GRAVITY * PHYSICS_FRAME_TIME;

        let mut new_pos = ChunkVec3(
            pos.0 + Vec3::new(0.0, vel.y, 0.0) * PHYSICS_FRAME_TIME * MOVE_SPEED
        );
        if Self::check_collision(&new_pos, chunk) {
            vel.y = 0.0;
            new_pos = *pos;
        }

        new_pos = ChunkVec3(
            new_pos.0 + Vec3::new(vel.x, 0.0, 0.0) * PHYSICS_FRAME_TIME * MOVE_SPEED
        );
        if Self::check_collision(&new_pos, chunk) {
            vel.x = 0.0;
            new_pos = ChunkVec3(pos.0 + Vec3::new(0.0, new_pos.0.y - pos.0.y, 0.0));
        }

        new_pos = ChunkVec3(
            new_pos.0 + Vec3::new(0.0, 0.0, vel.z) * PHYSICS_FRAME_TIME * MOVE_SPEED
        );
        if Self::check_collision(&new_pos, chunk) {
            vel.z = 0.0;
            new_pos = ChunkVec3(
                pos.0 + Vec3::new(new_pos.0.x - pos.0.x, new_pos.0.y - pos.0.y, 0.0)
            );
        }
        println!("New POS {:?}", new_pos);
        println!("NEW POS {:?}", new_pos.to_chunk());
        *pos = new_pos;
    }

    fn check_collision(
        pos: &ChunkVec3,
        chunk: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) -> bool {
        if pos.0.x < 0.0 || pos.0.z < 0.0 || pos.0.y < 0.0
        {
            return true;
        }
        let chunk_pos = pos.to_chunk(); // only cast if we know its a safe usize
        if
            chunk_pos.x >= CHUNK_SIZE ||
            chunk_pos.z >= CHUNK_SIZE ||
            chunk_pos.y >= CHUNK_SIZE
        {
            return true;
        }
        // Check if the entity at this position is solid
        match chunk[chunk_pos.x as usize][chunk_pos.z as usize][chunk_pos.y as usize] {
            EntityType::SolidBlock => true,
            _ => false,
        }
    }
}
