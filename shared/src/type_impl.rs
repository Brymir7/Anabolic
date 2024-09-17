use std::ops::Add;

use macroquad::math::{vec3, Vec3};

use crate::{
    config::{CHUNK_SIZE, INITIAL_PLAYER_POS},
    types::{
        AnimationCallbackEvent, AnimationState, ChunkPos, ChunkVec3, CurrWeapon, Enemies, EnemyHandle, EnemyType, EntityType, MaxWeapon, Player, PossibleEnemySizes, SolidBlocks, Weapon, WeaponType, WorldEvent
    },
};

impl Enemies {
    pub fn new() -> Self {
        Enemies {
            positions: Vec::new(),
            velocities: Vec::new(),
            animation_state: Vec::new(),
            size: Vec::new(),
            healths: Vec::new(),
            e_type: Vec::new(),
        }
    }
    pub fn new_enemy(
        &mut self,
        pos: ChunkVec3,
        vel: Vec3,
        size: PossibleEnemySizes,
        health: u8,
        e_type: EnemyType,
    ) -> EnemyHandle {
        for (idx, curr_e_type) in self.e_type.iter_mut().enumerate() {
            match curr_e_type {
                EnemyType::Empty => {
                    *curr_e_type = e_type;
                    self.positions[idx] = pos;
                    self.velocities[idx] = vel;
                    self.animation_state.push(AnimationState::default());
                    self.size[idx] = size;
                    self.healths[idx] = health;
                    return EnemyHandle(idx as u16);
                }
                _ => {continue;}
            }
        }
        self.positions.push(pos);
        self.velocities.push(vel);
        self.animation_state.push(AnimationState::default());
        self.size.push(size);
        self.healths.push(health);
        self.e_type.push(e_type);
        return EnemyHandle((self.positions.len() - 1) as u16);
    }
    pub fn get_vec3_size(size: PossibleEnemySizes) -> Vec3 {
        match size {
            PossibleEnemySizes::SMALL => Vec3::splat(0.25),
            PossibleEnemySizes::MEDIUM => Vec3::splat(0.5),
            PossibleEnemySizes::LARGE => Vec3::splat(0.75),
            PossibleEnemySizes::BOSS => Vec3::splat(1.25),
        }
    }
    pub fn get_hitbox_from_size(size: PossibleEnemySizes) -> Vec3 {
        match size {
            PossibleEnemySizes::SMALL => Vec3::splat(0.25) * 2.0,
            PossibleEnemySizes::MEDIUM => Vec3::splat(0.5) * 2.0,
            PossibleEnemySizes::LARGE => Vec3::splat(0.75) * 2.0,
            PossibleEnemySizes::BOSS => Vec3::splat(1.25) * 2.0,
        }
    }
    pub fn remove_enemy(&mut self, h: EnemyHandle) {
        let index = h.0 as usize;
        if index < self.positions.len() {
            self.e_type[index] = EnemyType::Empty;
        }
    }
    pub fn get_occupied_tiles(pos: &ChunkVec3, half_hitbox: &Vec3) -> Vec<ChunkPos> {
        let mut res =Vec::new();
        let start = ChunkVec3(pos.0 - *half_hitbox).to_chunk();
        let end = ChunkVec3(pos.0 + *half_hitbox).to_chunk();
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                for z in start.z..=end.z {
                    if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
                        res.push(ChunkPos::new(x, y, z));
                    }
                }
            }
        }

        res
    } 
}

impl SolidBlocks {
    pub fn new() -> Self {
        SolidBlocks {
            positions: Vec::new(),
        }
    }
    pub fn new_block(&mut self, pos: ChunkVec3) {
        self.positions.push(pos);
    }
}
impl Default for AnimationState {
    fn default() -> Self {
        AnimationState {
            current_step: 0.0,
            max_step: 1.0,
            callback: AnimationCallbackEvent::None,
        }
    }
}

impl Add<Vec3> for ChunkVec3 {
    type Output = ChunkVec3;

    fn add(self, rhs: Vec3) -> ChunkVec3 {
        ChunkVec3(self.0 + rhs)
    }
}
impl Weapon {
    fn new_shotgun() -> Self {
        Weapon {
            damage: 1.0,
            reload_speed: 0.5,
            time_last_reload: 0.0,
            w_type: WeaponType::Shotgun,
        }
    }
}
impl Default for Player {
    fn default() -> Self {
        Player {
            pos: ChunkVec3(INITIAL_PLAYER_POS),
            vel: Vec3::ZERO,
            yaw: 0.77,
            pitch: 0.0,
            weapon_unlocked: MaxWeapon(1),
            curr_weapon: CurrWeapon(0),
            weapons: [Weapon::new_shotgun()],
            animation_state: AnimationState::default(),
        }
    }
}

impl Player {
    pub fn get_current_weapon(&self) -> &Weapon {
        &self.weapons[self.curr_weapon.0]
    }
    pub fn swap_to_weapon(&mut self, w_type: WeaponType) {
        match w_type {
            WeaponType::Shotgun => {
                self.curr_weapon = CurrWeapon(0);
            }
        }
    }
    pub fn swap_next_weapon(&mut self) {
        self.curr_weapon = CurrWeapon((self.curr_weapon.0 + 1) % (self.weapon_unlocked.0 - 1));
    }

}

impl ChunkPos {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        ChunkPos {
            x,
            y,
            z,
        }
    }
    pub fn as_vec3(&self) -> Vec3{
        return vec3(self.x as f32, self.y as f32, self.z as f32);
    }
}
impl ChunkVec3 {
    pub fn to_chunk(&self) -> ChunkPos {
        let data = self.0;
        let data = data.clamp(Vec3::splat(0.0), Vec3::splat(CHUNK_SIZE as f32 - 0.51)); // small enough to not get rounded to chunk size
        debug_assert!(data.x.round() < 255.0 && data.x >= 0.0);
        debug_assert!(data.y.round() < 255.0 && data.y >= 0.0);
        debug_assert!(data.z.round() < 255.0 && data.z >= 0.0);
        return ChunkPos::new(data.x.round() as u8, data.y.round() as u8, data.z.round() as u8);
    }
}

