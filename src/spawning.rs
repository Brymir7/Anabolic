use std::time::Duration;

use macroquad::rand;
use shared::{
    config::{CHUNK_SIZE, WORLD_BORDER},
    types::{ ChunkVec3, EnemyType, EntityType, PossibleEnemySizes },
    Vec3,
};

use crate::World;

pub struct SpawnConfig {
    pub enemies_per_minute: usize,
    pub size_weights: [f32; 4], // Weights for SMALL, MEDIUM, LARGE, BOSS
    pub boss_spawn_minutes: Vec<u32>,
}

pub struct SpawningSystem {
    pub elapsed_time: Duration,
    pub difficulty_multiplier: f32,
    pub current_minute: u32,
    pub spawn_configs: Vec<SpawnConfig>,
    pub time_since_last_spawn: Duration,
    pub enemies_spawned_this_minute: usize,
}

impl SpawningSystem {
    pub fn new() -> Self {
        Self {
            elapsed_time: Duration::from_secs(0),
            difficulty_multiplier: 1.0,
            current_minute: 0,
            spawn_configs: vec![
                // Minute 0
                SpawnConfig {
                    enemies_per_minute: 28,
                    size_weights: [0.8, 0.2, 0.0, 0.0],
                    boss_spawn_minutes: vec![],
                },
                // Minute 1
                SpawnConfig {
                    enemies_per_minute: 35,
                    size_weights: [0.75, 0.25, 0.0, 0.0],
                    boss_spawn_minutes: vec![],
                },
                // Minute 2
                SpawnConfig {
                    enemies_per_minute: 42,
                    size_weights: [0.7, 0.3, 0.0, 0.0],
                    boss_spawn_minutes: vec![],
                },
                // Minute 3
                SpawnConfig {
                    enemies_per_minute: 45,
                    size_weights: [0.65, 0.3, 0.05, 0.0],
                    boss_spawn_minutes: vec![],
                },
                // Minute 4
                SpawnConfig {
                    enemies_per_minute: 45,
                    size_weights: [0.6, 0.35, 0.05, 0.0],
                    boss_spawn_minutes: vec![],
                },
                // Minute 5 (First boss)
                SpawnConfig {
                    enemies_per_minute: 51,
                    size_weights: [0.55, 0.35, 0.1, 0.0],
                    boss_spawn_minutes: vec![5],
                },
                // Minute 6
                SpawnConfig {
                    enemies_per_minute: 60,
                    size_weights: [0.5, 0.4, 0.1, 0.0],
                    boss_spawn_minutes: vec![],
                },
                // Minute 7
                SpawnConfig {
                    enemies_per_minute: 72,
                    size_weights: [0.45, 0.4, 0.15, 0.0],
                    boss_spawn_minutes: vec![],
                },
                // Minute 8
                SpawnConfig {
                    enemies_per_minute: 80,
                    size_weights: [0.4, 0.45, 0.15, 0.0],
                    boss_spawn_minutes: vec![],
                },
                // Minute 9
                SpawnConfig {
                    enemies_per_minute: 50,
                    size_weights: [0.35, 0.45, 0.2, 0.0],
                    boss_spawn_minutes: vec![],
                },
                // Minute 10 (Second boss)
                SpawnConfig {
                    enemies_per_minute: 3,
                    size_weights: [0.3, 0.45, 0.25, 0.0],
                    boss_spawn_minutes: vec![10],
                }
                // ... Add more configurations up to 30 minutes
            ],
            time_since_last_spawn: Duration::from_secs(0),
            enemies_spawned_this_minute: 0,
        }
    }

    pub fn update(&mut self, delta_time: Duration, world: &mut World) {
        self.elapsed_time += delta_time;
        self.time_since_last_spawn += delta_time;

        let new_minute = (self.elapsed_time.as_secs() / 60) as u32;

        if new_minute > self.current_minute {
            self.current_minute = new_minute;
            self.difficulty_multiplier *= 1.1; // Increase difficulty by 10% each minute
            self.enemies_spawned_this_minute = 0;
        }

        let config =
            &self.spawn_configs
                [self.current_minute.min((self.spawn_configs.len() as u32) - 1) as usize];

        // Calculate spawn interval
        let spawn_interval = Duration::from_secs(60) / (config.enemies_per_minute as u32);

        // Spawn an enemy if enough time has passed and we haven't spawned all enemies for this minute
        if
            self.time_since_last_spawn >= spawn_interval &&
            self.enemies_spawned_this_minute < config.enemies_per_minute
        {
            self.spawn_enemy(world, config);
            self.time_since_last_spawn = Duration::from_secs(0);
            self.enemies_spawned_this_minute += 1;
        }

        // Spawn boss if it's a boss spawn minute and we haven't spawned all enemies yet
        if
            config.boss_spawn_minutes.contains(&self.current_minute) &&
            self.enemies_spawned_this_minute == 0
        {
            self.spawn_boss(world);
        }
    }

    fn spawn_enemy(&self, world: &mut World, config: &SpawnConfig) {
        let size = self.get_random_size(&config.size_weights);
        let position = self.get_random_position_ground_enemy();
        let velocity = self.get_random_velocity();
        let health = self.get_health_based_on_size(size);
        let enemy_index = world.enemies.new_enemy(
            position,
            velocity,
            size,
            health,
            EnemyType::Regular
        );

        // Place the enemy in the world layout
        let chunk_pos = position.0;
        world.world_layout[chunk_pos.x as usize][chunk_pos.y as usize][chunk_pos.z as usize].push(
            EntityType::Enemy(enemy_index)
        );
    }
    fn get_health_based_on_size(&self, size: PossibleEnemySizes) -> u8 {
        match size {
            PossibleEnemySizes::SMALL => {
                return 1;
            }
            PossibleEnemySizes::BOSS => {
                return 10;
            }
            PossibleEnemySizes::LARGE => {
                return 5;
            }
            PossibleEnemySizes::MEDIUM => {
                return 3;
            }
        }
    }
    fn get_random_size(&self, weights: &[f32; 4]) -> PossibleEnemySizes {
        let random_value: f32 = rand::gen_range(0.0, 1.0);

        let mut cumulative_weight = 0.0;

        for (i, &weight) in weights.iter().enumerate() {
            cumulative_weight += weight;
            if random_value <= cumulative_weight {
                return match i {
                    0 => PossibleEnemySizes::SMALL,
                    1 => PossibleEnemySizes::MEDIUM,
                    2 => PossibleEnemySizes::LARGE,
                    _ => PossibleEnemySizes::BOSS,
                };
            }
        }

        PossibleEnemySizes::SMALL // Default case
    }

    fn get_random_position_ground_enemy(&self) -> ChunkVec3 {
        let border_threshold = WORLD_BORDER + 2.0; // Distance from the border where enemies can spawn
        let x = if rand::gen_range(0.0, 1.0) > 0.5 {
            rand::gen_range(0.0, border_threshold)
        } else {
            rand::gen_range((CHUNK_SIZE as f32) - border_threshold, CHUNK_SIZE as f32 - WORLD_BORDER)
        };
        let z = if rand::gen_range(0.0, 1.0) > 0.5 {
            rand::gen_range(0.0, border_threshold)
        } else {
            rand::gen_range((CHUNK_SIZE as f32) - border_threshold, CHUNK_SIZE as f32 - WORLD_BORDER)
        };
    
        ChunkVec3(
            Vec3::new(x, 8.0, z)
        )
    }
    

    fn get_random_velocity(&self) -> Vec3 {
        // Implement logic to get a random velocity
        Vec3::new(rand::gen_range(-1.0, 1.0), 0.0, rand::gen_range(-1.0, 1.0))
    }

    fn spawn_boss(&self, world: &mut World) {
        let position = self.get_random_position_ground_enemy();
        let velocity = self.get_random_velocity();
        let health = self.get_health_based_on_size(PossibleEnemySizes::BOSS);
        let boss_index = world.enemies.new_enemy(
            position,
            velocity,
            PossibleEnemySizes::BOSS,
            health,
            EnemyType::Regular
        );

        let chunk_pos = position.0;
        world.world_layout[chunk_pos.x as usize][chunk_pos.y as usize][chunk_pos.z as usize].push(
            EntityType::Enemy(boss_index)
        );
    }
}

pub fn update_spawning_system(
    world: &mut World,
    spawning_system: &mut SpawningSystem,
    delta_time: Duration
) {
    spawning_system.update(delta_time, world);
}
