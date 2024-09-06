use shared::{
    config::{ CHUNK_SIZE, ENEMY_DEFAULT_MOVE_SPEED, GRAVITY, MOVE_SPEED, PHYSICS_FRAME_TIME },
    types::{ ChunkVec3, EnemyHandle, EntityType, PossibleEnemySizes, RegularEnemies },
    vec3,
    Vec3,
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
        if Self::check_collision(&(new_pos + vec3(0.0, -1.0, 0.0)), chunk) {
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
        *pos = new_pos;
    }
    pub fn update_ground_enemies(
        positions: &mut Vec<ChunkVec3>,
        velocities: &mut Vec<Vec3>,
        sizes: &Vec<PossibleEnemySizes>,
        chunk: &mut [[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) {
        // Clear previous enemy positions
        Self::clear_enemy_positions(chunk);

        for (i, pos) in positions.iter_mut().enumerate() {
            let hitbox = RegularEnemies::get_hitbox_from_size(sizes[i]);
            let mut vel = velocities[i];
            vel.y += GRAVITY * PHYSICS_FRAME_TIME;

            let mut new_pos = ChunkVec3(
                pos.0 + Vec3::new(0.0, vel.y, 0.0) * PHYSICS_FRAME_TIME * ENEMY_DEFAULT_MOVE_SPEED
            );
            if Self::check_collision_multi_tile(&new_pos, &hitbox, chunk) {
                vel.y = 0.0;
                new_pos = *pos;
            }

            new_pos = ChunkVec3(
                new_pos.0 +
                    Vec3::new(vel.x, 0.0, 0.0) * PHYSICS_FRAME_TIME * ENEMY_DEFAULT_MOVE_SPEED
            );
            if Self::check_collision_multi_tile(&new_pos, &hitbox, chunk) {
                vel.x = 0.0;
                new_pos = ChunkVec3(pos.0 + Vec3::new(0.0, new_pos.0.y - pos.0.y, 0.0));
            }

            new_pos = ChunkVec3(
                new_pos.0 +
                    Vec3::new(0.0, 0.0, vel.z) * PHYSICS_FRAME_TIME * ENEMY_DEFAULT_MOVE_SPEED
            );
            if Self::check_collision_multi_tile(&new_pos, &hitbox, chunk) {
                vel.z = 0.0;
                new_pos = ChunkVec3(
                    pos.0 + Vec3::new(new_pos.0.x - pos.0.x, new_pos.0.y - pos.0.y, 0.0)
                );
            }
            *pos = new_pos;

            // Update world_layout with new enemy position
            Self::update_enemy_position(
                chunk,
                EntityType::RegularEnemy(EnemyHandle(i as u16)),
                &new_pos,
                &hitbox
            );
        }
    }

    pub fn update_flying_enemies(
        positions: &mut Vec<ChunkVec3>,
        velocities: &mut Vec<Vec3>,
        sizes: &Vec<PossibleEnemySizes>,
        chunk: &mut [[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) {
        // Clear previous enemy positions
        Self::clear_enemy_positions(chunk);

        for (i, pos) in positions.iter_mut().enumerate() {
            let hitbox = RegularEnemies::get_hitbox_from_size(sizes[i]);
            let mut vel = velocities[i];
            let mut new_pos = ChunkVec3(
                pos.0 + Vec3::new(0.0, vel.y, 0.0) * PHYSICS_FRAME_TIME * ENEMY_DEFAULT_MOVE_SPEED
            );
            if Self::check_collision_multi_tile(&new_pos, &hitbox, chunk) {
                vel.y = 0.0;
                new_pos = *pos;
            }

            new_pos = ChunkVec3(
                new_pos.0 +
                    Vec3::new(vel.x, 0.0, 0.0) * PHYSICS_FRAME_TIME * ENEMY_DEFAULT_MOVE_SPEED
            );
            if Self::check_collision_multi_tile(&new_pos, &hitbox, chunk) {
                vel.x = 0.0;
                new_pos = ChunkVec3(pos.0 + Vec3::new(0.0, new_pos.0.y - pos.0.y, 0.0));
            }

            new_pos = ChunkVec3(
                new_pos.0 +
                    Vec3::new(0.0, 0.0, vel.z) * PHYSICS_FRAME_TIME * ENEMY_DEFAULT_MOVE_SPEED
            );
            if Self::check_collision_multi_tile(&new_pos, &hitbox, chunk) {
                vel.z = 0.0;
                new_pos = ChunkVec3(
                    pos.0 + Vec3::new(new_pos.0.x - pos.0.x, new_pos.0.y - pos.0.y, 0.0)
                );
            }
            *pos = new_pos;

            // Update world_layout with new enemy position
            Self::update_enemy_position(
                chunk,
                EntityType::FlyingEnemy(EnemyHandle(i as u16)),
                &new_pos,
                &hitbox
            );
        }
    }

    fn check_collision_multi_tile(
        pos: &ChunkVec3,
        hitbox: &Vec3,
        chunk: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) -> bool {
        let start = ChunkVec3(pos.0 - *hitbox * 0.5).to_chunk();
        let end = ChunkVec3(pos.0 + *hitbox * 0.5).to_chunk();

        for x in start.x..=end.x {
            for y in start.y..=end.y {
                for z in start.z..=end.z {
                    if
                        x < 0 ||
                        y < 0 ||
                        z < 0 ||
                        x >= CHUNK_SIZE ||
                        y >= CHUNK_SIZE ||
                        z >= CHUNK_SIZE
                    {
                        return true;
                    }
                    match chunk[x as usize][z as usize][y as usize] {
                        EntityType::SolidBlock => {
                            return true;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
        }
        false
    }

    fn clear_enemy_positions(
        chunk: &mut [[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
    ) {
        for x in 0..CHUNK_SIZE as usize {
            for y in 0..CHUNK_SIZE as usize {
                for z in 0..CHUNK_SIZE as usize {
                    if let EntityType::FlyingEnemy(_) = chunk[x][z][y] {
                        chunk[x][z][y] = EntityType::None;
                    }
                }
            }
        }
    }

    fn update_enemy_position(
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
                    if
                        x >= 0 &&
                        y >= 0 &&
                        z >= 0 &&
                        x < CHUNK_SIZE &&
                        y < CHUNK_SIZE &&
                        z < CHUNK_SIZE
                    {
                        chunk[x as usize][z as usize][y as usize] = entity_type;
                    }
                }
            }
        }
    }
    fn check_collision(
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
        match chunk[chunk_pos.x as usize][chunk_pos.z as usize][chunk_pos.y as usize] {
            EntityType::SolidBlock => true,
            EntityType::RegularEnemy(_) => true,
            EntityType::FlyingEnemy(_) => true,
            EntityType::InteractableBlock(_) => true,
            EntityType::Player => true,
            EntityType::None => false,
        }
    }
}
