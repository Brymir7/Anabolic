use std::collections::VecDeque;

use shared::{
    config::CHUNK_SIZE,
    types::{
        ChunkPos, ChunkVec3, EnemyHandle, EntityType
    },
    Vec3,
    BLUE,
    RED, // dont use macroquad types here, then avoid dependency and then we could make it compile quicker ?
};

use crate::Screen;

#[no_mangle]
pub fn render_enemy_world_positions(
    screen: &Screen,
    world_layout: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    flying_enemies: &[ChunkVec3],
    regular_enemies: &[ChunkVec3],
) {
    for (handle, &position) in flying_enemies.iter().enumerate() {
       render_enemy(screen, world_layout, position, EnemyHandle(handle as u16), true);
    }

    for (handle, &position) in regular_enemies.iter().enumerate() {
        render_enemy(screen, world_layout, position, EnemyHandle(handle as u16), false);
    }
}

fn render_enemy(
    screen: & Screen,
    world_layout: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    position: ChunkVec3,
    handle: EnemyHandle,
    is_flying: bool,
) {
    let mut visited = [[[false; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
    let mut queue = VecDeque::new();
    let mut occupied_tiles = Vec::new();

    let start_pos = position.to_chunk();
    queue.push_back(start_pos);
    visited[start_pos.x as usize][start_pos.y as usize][start_pos.z as usize] = true;
    occupied_tiles.push(start_pos);

    while let Some(current_pos) = queue.pop_front() {
        for neighbor in get_neighbors(current_pos) {
            if is_valid_position(neighbor) &&
               !visited[neighbor.x as usize][neighbor.y as usize][neighbor.z as usize] &&
               is_enemy_tile(world_layout, neighbor, handle, is_flying)
            {
                visited[neighbor.x as usize][neighbor.y as usize][neighbor.z as usize] = true;
                queue.push_back(neighbor);
                occupied_tiles.push(neighbor);
            }
        }
    }

    if !occupied_tiles.is_empty() {
        let (min_pos, max_pos) = calculate_bounding_box(&occupied_tiles);
        let center = (min_pos + max_pos) * 0.5;
        let size = max_pos - min_pos + Vec3::new(1.0, 1.0, 1.0); // Add 1 to include the last tile

        let color = if is_flying { BLUE } else { RED };
        screen.drawer.draw_cube_wires(center, size, color);
    }
}

fn is_enemy_tile(
    world_layout: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    pos: ChunkPos,
    handle: EnemyHandle,
    is_flying: bool,
) -> bool {
    match world_layout[pos.x as usize][pos.y as usize][pos.z as usize] {
        EntityType::FlyingEnemy(h) if is_flying && h == handle => true,
        EntityType::RegularEnemy(h) if !is_flying && h == handle => true,
        _ => false,
    }
}

fn get_neighbors(pos: ChunkPos) -> [ChunkPos; 6] {
    [
        ChunkPos::new(pos.x.wrapping_add(1), pos.y, pos.z),
        ChunkPos::new(pos.x.wrapping_sub(1), pos.y, pos.z),
        ChunkPos::new(pos.x, pos.y.wrapping_add(1), pos.z),
        ChunkPos::new(pos.x, pos.y.wrapping_sub(1), pos.z),
        ChunkPos::new(pos.x, pos.y, pos.z.wrapping_add(1)),
        ChunkPos::new(pos.x, pos.y, pos.z.wrapping_sub(1)),
    ]
}

fn is_valid_position(pos: ChunkPos) -> bool {
    pos.x < CHUNK_SIZE as u8 && pos.y < CHUNK_SIZE as u8 && pos.z < CHUNK_SIZE as u8
}

fn calculate_bounding_box(tiles: &[ChunkPos]) -> (Vec3, Vec3) {
    let mut min_pos = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max_pos = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

    for tile in tiles {
        min_pos.x = min_pos.x.min(tile.x as f32);
        min_pos.y = min_pos.y.min(tile.y as f32);
        min_pos.z = min_pos.z.min(tile.z as f32);

        max_pos.x = max_pos.x.max(tile.x as f32);
        max_pos.y = max_pos.y.max(tile.y as f32);
        max_pos.z = max_pos.z.max(tile.z as f32);
    }

    (min_pos, max_pos)
}