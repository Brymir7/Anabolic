use macroquad::{color::GREEN, math::{vec3, Vec3}, models::draw_cube_wires};

use crate::{config::{CHUNK_SIZE, TILE_SIZE}, types::EntityType};

pub struct RenderSystem;
impl RenderSystem {
    pub fn render_world(world_layout: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]) {
        for x in 0..world_layout.len() {
            for z in 0..world_layout.len() {
                for y in 0..world_layout.len() {
                    match world_layout[x][z][y] {
                        EntityType::SolidBlock => {
                            draw_cube_wires(vec3(x as f32, y as f32, z as f32), Vec3::splat(TILE_SIZE), GREEN);
                        }                        
                        _ => {}
                    }
                }
            }
        }
    }
}