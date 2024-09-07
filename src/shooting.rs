use shared::{ config::CHUNK_SIZE, types::{ ChunkVec3, EntityType }, Vec3 };

pub fn shotgun_shoot(
    origin: ChunkVec3,
    target_dir: Vec3,
    world_layout: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
) {
    println!("target dir {}", target_dir);
    let relative_chunk_dist_x = 1.0 / target_dir.x.abs();
    let relative_chunk_dist_y = 1.0 / target_dir.y.abs();
    let relative_chunk_dist_z = 1.0 / target_dir.z.abs();

    let step_x: isize = if target_dir.x > 0.0 { 1 } else { -1 };
    let step_y: isize = if target_dir.y > 0.0 { 1 } else { -1 };
    let step_z: isize = if target_dir.z > 0.0 { 1 } else { -1 };

    let curr_chunk = origin.to_chunk();
    let mut curr_map_tile_x = curr_chunk.x;
    let mut curr_map_tile_y = curr_chunk.y;
    let mut curr_map_tile_z = curr_chunk.z;

    let mut dist_side_x = if target_dir.x < 0.0 {
        (origin.0.x - (curr_map_tile_x as f32)) * relative_chunk_dist_x
    } else {
        ((curr_map_tile_x as f32) + 1.0 - origin.0.x) * relative_chunk_dist_x
    };
    let mut dist_side_y = if target_dir.y < 0.0 {
        (origin.0.y - (curr_map_tile_y as f32)) * relative_chunk_dist_y
    } else {
        ((curr_map_tile_y as f32) + 1.0 - origin.0.y) * relative_chunk_dist_y
    };
    let mut dist_side_z = if target_dir.z < 0.0 {
        (origin.0.z - (curr_map_tile_z as f32)) * relative_chunk_dist_z
    } else {
        ((curr_map_tile_z as f32) + 1.0 - origin.0.z) * relative_chunk_dist_z
    };

    while
        curr_map_tile_x < CHUNK_SIZE &&
        curr_map_tile_y < CHUNK_SIZE &&
        curr_map_tile_z < CHUNK_SIZE
    {
        // Check for collision with an entity
        let entity =
            world_layout[curr_map_tile_x as usize][curr_map_tile_y as usize]
                [curr_map_tile_z as usize];
        if
            entity != EntityType::None &&
            entity != EntityType::SolidBlock &&
            entity != EntityType::Player
        {
            println!(
                "Hit entity {:?} at ({}, {}, {})",
                entity,
                curr_map_tile_x,
                curr_map_tile_y,
                curr_map_tile_z
            );
            break;
        }


        if dist_side_x < dist_side_y && dist_side_x < dist_side_z {
            // Cross the YZ plane
            curr_map_tile_x = ((curr_map_tile_x as isize) + step_x) as u8;
            dist_side_x += relative_chunk_dist_x;
        } else if dist_side_y < dist_side_z {
            // Cross the XZ plane
            curr_map_tile_y = ((curr_map_tile_y as isize) + step_y) as u8;
            dist_side_y += relative_chunk_dist_y;
        } else {
            // Cross the XY plane
            curr_map_tile_z = ((curr_map_tile_z as isize) + step_z) as u8;
            dist_side_z += relative_chunk_dist_z;
        }
    }

    println!(
        "Shot exited the chunk at ({}, {}, {})",
        curr_map_tile_x,
        curr_map_tile_y,
        curr_map_tile_z
    );
}
