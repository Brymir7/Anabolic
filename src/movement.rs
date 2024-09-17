use shared::{
    config::{ CHUNK_SIZE, GRAVITY, MOVE_SPEED, PHYSICS_FRAME_TIME },
    types::{ ChunkPos, ChunkVec3, Enemies, EnemyHandle, EnemyType, EntityType, PossibleEnemySizes },
    vec3,
    Vec3,
};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update_player(
        pos: &mut ChunkVec3,
        vel: &mut Vec3,
        enemies: &Enemies,
        chunk: &mut [
            [[Vec<EntityType>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
            CHUNK_SIZE as usize
        ]
    ) {
        vel.y += GRAVITY * PHYSICS_FRAME_TIME;

        let mut new_pos = ChunkVec3(
            pos.0 + Vec3::new(0.0, vel.y, 0.0) * PHYSICS_FRAME_TIME * MOVE_SPEED
        );
        if Self::check_collision_player(&(new_pos + vec3(0.0, -1.0, 0.0)), enemies, chunk) {
            vel.y = 0.0;
            new_pos = *pos;
        }

        new_pos = ChunkVec3(
            new_pos.0 + Vec3::new(vel.x, 0.0, 0.0) * PHYSICS_FRAME_TIME * MOVE_SPEED
        );
        if Self::check_collision_player(&new_pos, enemies, chunk) {
            vel.x = 0.0;
            new_pos = ChunkVec3(pos.0 + Vec3::new(0.0, new_pos.0.y - pos.0.y, 0.0));
        }

        new_pos = ChunkVec3(
            new_pos.0 + Vec3::new(0.0, 0.0, vel.z) * PHYSICS_FRAME_TIME * MOVE_SPEED
        );
        if Self::check_collision_player(&new_pos, enemies, chunk) {
            vel.z = 0.0;
            new_pos = ChunkVec3(
                pos.0 + Vec3::new(new_pos.0.x - pos.0.x, new_pos.0.y - pos.0.y, 0.0)
            );
        }
        new_pos.0 = new_pos.0.clamp(Vec3::splat(1.0), Vec3::splat((CHUNK_SIZE as f32) - 1.0));
        *pos = new_pos;

        Self::update_world_position(chunk, EntityType::Player, &new_pos, &Vec3::splat(0.5));
    }

    pub fn update_enemies(
        player_pos: &ChunkVec3,
        enemies: &mut Enemies,
        chunk: &mut [
            [[Vec<EntityType>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
            CHUNK_SIZE as usize
        ]
    ) {
        for i in 0..enemies.positions.len() {
            let (left, right) = enemies.positions.split_at_mut(i);
            let (current, right) = right.split_at_mut(1);
            let pos = &mut current[0];
            let initial_pos = pos.clone();
            let other_positions: Vec<ChunkVec3> = left
                .iter()
                .chain([initial_pos].iter()) // to upkeep valid handles
                .chain(right.iter())
                .cloned()
                .collect();

            let enemy_handle = EnemyHandle(i as u16);
            let half_hitbox = Enemies::get_hitbox_from_size(enemies.size[i]) * 0.5;
            let mut vel = enemies.velocities[i];
            if enemies.e_type[i] == EnemyType::Empty {
                continue;
            }
            vel.y += GRAVITY * PHYSICS_FRAME_TIME;
            // vel.x = (player_pos.0.x - pos.0.x) * 0.3; // make farther enemies quicker, but dont overdo it
            // vel.z = (player_pos.0.z - pos.0.z) * 0.3;

            const MAX_XYZ: Vec3 = Vec3::splat((CHUNK_SIZE as f32) - 1.51); // small enough to not get rounded to chunk size
            let x_border = pos.0.x + half_hitbox.x * vel.x.signum();
            let curr_pos = ChunkVec3(
                Vec3::new(x_border + vel.x * PHYSICS_FRAME_TIME, pos.0.y, pos.0.z)
            );
            if
                curr_pos.0.x < MAX_XYZ.x &&
                Self::enemy_check_if_chunk_is_valid_pos(
                    curr_pos,
                    enemy_handle,
                    &half_hitbox,
                    &other_positions,
                    &enemies.size,
                    &enemies.e_type,
                    chunk
                )
            {
                pos.0.x = curr_pos.0.x - half_hitbox.x * vel.x.signum();
            }
            let y_border = pos.0.y + half_hitbox.y * vel.y.signum();
            let curr_pos = ChunkVec3(
                Vec3::new(pos.0.x, y_border + vel.y * PHYSICS_FRAME_TIME, pos.0.z)
            );
            if
                curr_pos.0.y < MAX_XYZ.y &&
                Self::enemy_check_if_chunk_is_valid_pos(
                    curr_pos,
                    enemy_handle,
                    &half_hitbox,
                    &other_positions,
                    &enemies.size,
                    &enemies.e_type,
                    chunk
                )
            {
                pos.0.y = curr_pos.0.y - half_hitbox.y * vel.y.signum();
            }
            let z_border = pos.0.z + half_hitbox.z * vel.z.signum();
            let curr_pos = ChunkVec3(
                Vec3::new(pos.0.x, pos.0.y, z_border + vel.z * PHYSICS_FRAME_TIME)
            );
            if
                curr_pos.0.z < MAX_XYZ.z &&
                Self::enemy_check_if_chunk_is_valid_pos(
                    curr_pos,
                    enemy_handle,
                    &half_hitbox,
                    &other_positions,
                    &enemies.size,
                    &enemies.e_type,
                    chunk
                )
            {
                pos.0.z = curr_pos.0.z - half_hitbox.z * vel.z.signum();
            }
            let prev_tiles = &Enemies::get_occupied_tiles(&initial_pos, &half_hitbox);
            pos.0 = pos.0.clamp(vec3(1.0, 0.0, 1.0), Vec3::splat((CHUNK_SIZE as f32) - 1.0));
            Self::update_enemy_world_position(
                prev_tiles,
                &Enemies::get_occupied_tiles(&pos, &half_hitbox),
                chunk,
                EntityType::Enemy(enemy_handle)
            );
        }
    }

    fn enemy_check_if_chunk_is_valid_pos(
        pos: ChunkVec3,
        handle: EnemyHandle,
        half_hb1: &Vec3,
        other_positions: &Vec<ChunkVec3>,
        other_sizes: &Vec<PossibleEnemySizes>,
        other_types: &Vec<EnemyType>,
        chunk: &[[[Vec<EntityType>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) -> bool {
        let chunk_pos = pos.to_chunk();
        let entities_in_chunk =
            &chunk[chunk_pos.x as usize][chunk_pos.y as usize][chunk_pos.z as usize];

        for entity in entities_in_chunk {
            match entity {
                EntityType::Enemy(h_other) => {
                    if *h_other == handle {
                        continue;
                    }
                    if other_types[h_other.0 as usize] == EnemyType::Empty { // unnecessary, but keep to make sure it doesnt break, -> theres hould never be a reference to an empty enemy in the world layout
                        continue;
                    }
                    let half_hb2 =
                        Enemies::get_hitbox_from_size(other_sizes[h_other.0 as usize]) * 0.5;
                    let pos2 = other_positions[h_other.0 as usize];
                    if Self::intersect_hitbox(&pos.0, half_hb1, &pos2.0, &half_hb2) {
                        return false;
                    }
                }
                EntityType::Player => {
                    println!("handle collision ");
                }

                EntityType::SolidBlock | EntityType::InteractableBlock(_) => {
                    return false;
                }
            }
        }
        return true;
    }

    fn intersect_hitbox(p1: &Vec3, half_hb1: &Vec3, p2: &Vec3, half_hb2: &Vec3) -> bool {
        let x_overlap = (p1.x - p2.x).abs() <= half_hb1.x + half_hb2.x;
        let y_overlap = (p1.y - p2.y).abs() <= half_hb1.y + half_hb2.y;
        let z_overlap = (p1.z - p2.z).abs() <= half_hb1.z + half_hb2.z;
        x_overlap && y_overlap && z_overlap
    }

    fn update_world_position(
        chunk: &mut [
            [[Vec<EntityType>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
            CHUNK_SIZE as usize
        ],
        entity_type: EntityType,
        pos: &ChunkVec3,
        half_hitbox: &Vec3
    ) {
        let start = ChunkVec3(pos.0 - *half_hitbox).to_chunk();
        let end = ChunkVec3(pos.0 + *half_hitbox).to_chunk();
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                for z in start.z..=end.z {
                    if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
                        chunk[x as usize][y as usize][z as usize].push(entity_type);
                    }
                }
            }
        }
    }
    fn update_enemy_world_position(
        prev_tiles: &Vec<ChunkPos>,
        occupied_tiles: &Vec<ChunkPos>,
        chunk: &mut [
            [[Vec<EntityType>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
            CHUNK_SIZE as usize
        ],
        enemy_type: EntityType
    ) {
        debug_assert!(match enemy_type {
            EntityType::Enemy(_) => true,
            _ => false,
        });
        for tile in prev_tiles {
            chunk[tile.x as usize][tile.y as usize][tile.z as usize].retain(|e| *e != enemy_type);
        }
        for tile in occupied_tiles {
            chunk[tile.x as usize][tile.y as usize][tile.z as usize].push(enemy_type);
        }
    }

    fn player_check_if_chunk_is_valid_pos(
        pos: &ChunkVec3,
        half_hb1: &Vec3,
        enemy_pos: &Vec<ChunkVec3>,
        enemy_sizes: &Vec<PossibleEnemySizes>,
        chunk: &[[[Vec<EntityType>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) -> bool {
        let chunk_pos = pos.to_chunk();
        let entities_in_chunk =
            &chunk[chunk_pos.x as usize][chunk_pos.y as usize][chunk_pos.z as usize];

        for entity in entities_in_chunk {
            match entity {
                EntityType::Enemy(h_other) => {
                    let half_hb2 =
                        Enemies::get_hitbox_from_size(enemy_sizes[h_other.0 as usize]) * 0.5;
                    let pos2 = enemy_pos[h_other.0 as usize];
                    if Self::intersect_hitbox(&pos.0, half_hb1, &pos2.0, &half_hb2) {
                        return false;
                    }
                }
                EntityType::Player => {
                    continue;
                }
                EntityType::SolidBlock | EntityType::InteractableBlock(_) => {
                    return false;
                }
            }
        }
        return true;
    }
    fn check_collision_player(
        pos: &ChunkVec3,
        enemies: &Enemies,
        chunk: &[[[Vec<EntityType>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) -> bool {
        if pos.0.x < 0.0 || pos.0.z < 0.0 || pos.0.y < 0.0 {
            return true;
        }
        let chunk_pos = pos.to_chunk(); // only cast if we know its a safe usize
        if
            chunk_pos.x >= CHUNK_SIZE - 1 ||
            chunk_pos.z >= CHUNK_SIZE - 1 ||
            chunk_pos.y >= CHUNK_SIZE - 1
        {
            return true;
        }

        let res = Self::player_check_if_chunk_is_valid_pos(
            pos,
            &Vec3::splat(0.5),
            &enemies.positions,
            &enemies.size,
            chunk
        );
        !res
    }
}
