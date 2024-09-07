use shared::{
    config::CHUNK_SIZE,
    types::{
        ChunkVec3,
        EnemyIdentifier,
        EntityType,
        FlyingEnemies,
        Player,
        SolidBlocks,
        WeaponType,
        WorldEvent,
    },
    vec3,
    Vec3,
};

use crate::World;

pub fn shotgun_shoot(
    origin: ChunkVec3,
    target_dir: Vec3,
    world_layout: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
) -> Option<WorldEvent> {
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
        match entity {
            EntityType::FlyingEnemy(h) => {
                return Some(WorldEvent::HitEnemy(EnemyIdentifier::flying_enemy_identifier(h)));
            }
            EntityType::RegularEnemy(h) => {
                println!("hit regular enemy {:?}", h);
                return Some(WorldEvent::HitEnemy(EnemyIdentifier::regular_enemy_identifier(h)));
            }
            _ => {}
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

    return None;
}

pub fn shoot(
    player: &mut Player,
    world_layout: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
) -> Vec<WorldEvent> {
    let mut res = Vec::new();
    match player.get_current_weapon().w_type {
        WeaponType::Shotgun => {
            let front = vec3(
                player.yaw.cos() * player.pitch.cos(),
                player.pitch.sin(),
                player.yaw.sin() * player.pitch.cos()
            ).normalize();
            // make player a bit higher, so that when he looks down on smaller opponents he can hit them at their feet
            let event = shotgun_shoot(player.pos + vec3(0.0, 0.4, 0.0), front, world_layout);
            if let Some(event) = event {
                res.push(event);
            }
        }
    }
    res
}
