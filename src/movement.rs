use shared::{
    config::{ CHUNK_SIZE, ENEMY_DEFAULT_MOVE_SPEED, GRAVITY, MOVE_SPEED, PHYSICS_FRAME_TIME },
    types::{
        ChunkPos,
        ChunkVec1,
        ChunkVec3,
        EnemyHandle,
        EntityType,
        FlyingEnemies,
        PossibleEnemySizes,
        RegularEnemies,
        SolidBlocks,
    },
    vec3,
    Vec3,
};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update_player(
        pos: &mut ChunkVec3,
        vel: &mut Vec3,
        chunk: &mut [[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) {
        vel.y += GRAVITY * PHYSICS_FRAME_TIME;

        let mut new_pos = ChunkVec3(
            pos.0 + Vec3::new(0.0, vel.y, 0.0) * PHYSICS_FRAME_TIME * MOVE_SPEED
        );
        if Self::check_collision_player(&(new_pos + vec3(0.0, -1.0, 0.0)), chunk) {
            vel.y = 0.0;
            new_pos = *pos;
        }

        new_pos = ChunkVec3(
            new_pos.0 + Vec3::new(vel.x, 0.0, 0.0) * PHYSICS_FRAME_TIME * MOVE_SPEED
        );
        if Self::check_collision_player(&new_pos, chunk) {
            vel.x = 0.0;
            new_pos = ChunkVec3(pos.0 + Vec3::new(0.0, new_pos.0.y - pos.0.y, 0.0));
        }

        new_pos = ChunkVec3(
            new_pos.0 + Vec3::new(0.0, 0.0, vel.z) * PHYSICS_FRAME_TIME * MOVE_SPEED
        );
        if Self::check_collision_player(&new_pos, chunk) {
            vel.z = 0.0;
            new_pos = ChunkVec3(
                pos.0 + Vec3::new(new_pos.0.x - pos.0.x, new_pos.0.y - pos.0.y, 0.0)
            );
        }
        new_pos.0 = new_pos.0.clamp(Vec3::splat(0.0), Vec3::splat((CHUNK_SIZE as f32) - 1.0));
        *pos = new_pos;

        Self::update_world_position(chunk, EntityType::Player, &new_pos, &Vec3::splat(1.0));
    }

    pub fn update_ground_enemies(
        player_pos: &ChunkVec3,
        positions: &mut Vec<ChunkVec3>,
        velocities: &mut Vec<Vec3>,
        sizes: &Vec<PossibleEnemySizes>,
        chunk: &mut [[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) {
        Self::clear_enemy_positions(chunk); // TODO can be improved to only update existing enemies previous chunks,

        for (i, pos) in positions.iter_mut().enumerate() {
            let enemy_handle = EnemyHandle(i as u16);
            let hitbox = RegularEnemies::get_hitbox_from_size(sizes[i]);
            let mut vel = velocities[i];
            let enemy_chunk_pos = pos.to_chunk();

            vel.y += GRAVITY * PHYSICS_FRAME_TIME;
            vel.x = (player_pos.0.x - pos.0.x) * 0.2; // make farther enemies quicker, but dont overdo it
            vel.z = (player_pos.0.z - pos.0.z) * 0.2;

            let max_xyz = Vec3::splat((CHUNK_SIZE as f32) - 0.51); // small enough to not get rounded to chunk size
            let x_border = pos.0.x + 0.5 * hitbox.x * vel.x.signum();
            let new_pos_x = ChunkVec1(x_border + vel.x * PHYSICS_FRAME_TIME);
            if
                new_pos_x.0 < max_xyz.x &&
                Self::enemy_check_if_chunk_is_valid_pos(
                    enemy_handle,
                    ChunkPos {
                        x: new_pos_x.to_chunk_axis_idx(),
                        y: enemy_chunk_pos.y,
                        z: enemy_chunk_pos.z,
                    },
                    chunk
                )
            {
                pos.0.x = new_pos_x.0 - 0.5 * hitbox.x * vel.x.signum();
            }
            let y_border = pos.0.y + 0.5 * hitbox.y * vel.y.signum();
            let new_pos_y = ChunkVec1(y_border + vel.y * PHYSICS_FRAME_TIME);
            if
                new_pos_y.0 < max_xyz.y &&
                Self::enemy_check_if_chunk_is_valid_pos(
                    enemy_handle,
                    ChunkPos {
                        x: enemy_chunk_pos.x,
                        y: new_pos_y.to_chunk_axis_idx(),
                        z: enemy_chunk_pos.z,
                    },
                    chunk
                )
            {
                pos.0.y = new_pos_y.0 - 0.5 * hitbox.y * vel.y.signum();
            }
            let z_border = pos.0.z + 0.5 * hitbox.z * vel.z.signum();
            let new_pos_z = ChunkVec1(z_border + vel.z * PHYSICS_FRAME_TIME);

            if
                new_pos_z.0 < max_xyz.z &&
                Self::enemy_check_if_chunk_is_valid_pos(
                    enemy_handle,
                    ChunkPos {
                        x: enemy_chunk_pos.x,
                        y: enemy_chunk_pos.y,
                        z: new_pos_z.to_chunk_axis_idx(),
                    },
                    chunk
                )
            {
                pos.0.z = new_pos_z.0 - 0.5 * hitbox.z * vel.z.signum();
            }
            pos.0 = pos.0.clamp(Vec3::splat(0.0), Vec3::splat((CHUNK_SIZE as f32) - 1.0));
            Self::update_enemy_world_position(
                &RegularEnemies::get_occupied_tiles(&pos, &hitbox),
                chunk,
                EntityType::RegularEnemy(enemy_handle)
            );
        }
    }

    // pub fn update_flying_enemies(
    //     positions: &mut Vec<ChunkVec3>,
    //     velocities: &mut Vec<Vec3>,
    //     sizes: &Vec<PossibleEnemySizes>,
    //     chunk: &mut [[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    // ) {
    //     // Clear previous enemy positions
    //     Self::clear_enemy_positions(chunk);

    //     for (i, pos) in positions.iter_mut().enumerate() {
    //         let enemy_handle = EnemyHandle(i as u16);
    //         let hitbox = FlyingEnemies::get_hitbox_from_size(sizes[i]);
    //         let mut vel = velocities[i];

    //         let new_pos = pos;

    //     }
    // }

    fn enemy_check_if_chunk_is_valid_pos(
        handle: EnemyHandle,
        chunk_pos: ChunkPos,
        chunk: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) -> bool {
        match chunk[chunk_pos.x as usize][chunk_pos.y as usize][chunk_pos.z as usize] {
            EntityType::RegularEnemy(h) => { handle == h }
            EntityType::None => { true }
            _ => { false }
        }
    }

    fn clear_enemy_positions( // TODO only iterate over previous cells?
        chunk: &mut [[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) {
        for x in 0..CHUNK_SIZE as usize {
            for y in 0..CHUNK_SIZE as usize {
                for z in 0..CHUNK_SIZE as usize {
                    chunk[x][y][z] = match chunk[x][y][z] {
                        EntityType::FlyingEnemy(_) | EntityType::RegularEnemy(_) => {
                            EntityType::None
                        }
                        _ => {
                            continue;
                        }
                    };
                }
            }
        }
    }

    fn update_world_position(
        chunk: &mut [[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
        entity_type: EntityType,
        pos: &ChunkVec3,
        hitbox: &Vec3
    ) {
        let start = ChunkVec3(pos.0 - *hitbox * 0.5).to_chunk();
        let end = ChunkVec3(pos.0 + *hitbox * 0.5).to_chunk();
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                for z in start.z..=end.z {
                    if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
                        if chunk[x as usize][y as usize][z as usize] != EntityType::None {
                            continue;
                        }
                        chunk[x as usize][y as usize][z as usize] = entity_type;
                    }
                }
            }
        }
    }
    fn update_enemy_world_position(
        occupied_tiles: &Vec<ChunkPos>,
        chunk: &mut [[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
        enemy_type: EntityType
    ) {
        assert!(match enemy_type {
            EntityType::FlyingEnemy(_) => true,
            EntityType::RegularEnemy(_) => true,
            _ => false,
        });
        for tile in occupied_tiles {
            chunk[tile.x as usize][tile.y as usize][tile.z as usize] = enemy_type;
        }
    }
    fn check_collision_player(
        pos: &ChunkVec3,
        chunk: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) -> bool {
        if pos.0.x < 0.0 || pos.0.z < 0.0 || pos.0.y < 0.0 {
            return true;
        }
        let chunk_pos = pos.to_chunk(); // only cast if we know its a safe usize
        if chunk_pos.x >= CHUNK_SIZE || chunk_pos.z >= CHUNK_SIZE || chunk_pos.y >= CHUNK_SIZE {
            return true;
        }
        // Check if the entity at this position is solid
        match chunk[chunk_pos.x as usize][chunk_pos.y as usize][chunk_pos.z as usize] {
            EntityType::SolidBlock => true,
            EntityType::RegularEnemy(_) => true,
            EntityType::FlyingEnemy(_) => true,
            EntityType::InteractableBlock(_) => true,
            EntityType::Player => false,
            EntityType::None => false,
        }
    }
}
