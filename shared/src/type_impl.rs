use std::ops::Add;

use macroquad::math::Vec3;

use crate::{
    config::INITIAL_PLAYER_POS,
    types::{
        AnimationCallbackEvent, AnimationState, ChunkPos, ChunkVec1, ChunkVec3, CurrWeapon, EnemyHandle, FlyingEnemies, MaxWeapon, Player, PossibleEnemySizes, RegularEnemies, SolidBlocks, Weapon, WeaponType
    },
};

impl RegularEnemies {
    pub fn new() -> Self {
        RegularEnemies {
            positions: Vec::new(),
            velocities: Vec::new(),
            animation_state: Vec::new(),
            size: Vec::new(),
        }
    }
    pub fn new_enemy(
        &mut self,
        pos: ChunkVec3,
        vel: Vec3,
        size: PossibleEnemySizes
    ) -> EnemyHandle {
        self.positions.push(pos);
        self.velocities.push(vel);
        self.animation_state.push(AnimationState::default());
        self.size.push(size);
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
}
impl FlyingEnemies {
    pub fn new() -> Self {
        FlyingEnemies {
            positions: Vec::new(),
            velocities: Vec::new(),
            animation_state: Vec::new(),
            size: Vec::new(),
        }
    }
    pub fn new_enemy(
        &mut self,
        pos: ChunkVec3,
        vel: Vec3,
        size: PossibleEnemySizes
    ) -> EnemyHandle {
        self.positions.push(pos);
        self.velocities.push(vel);
        self.animation_state.push(AnimationState::default());
        self.size.push(size);
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
}
impl ChunkVec3 {
    pub fn to_chunk(&self) -> ChunkPos {
        let data = self.0;
        assert!(data.x.round() < 255.0 && data.x >= 0.0);
        assert!(data.y.round() < 255.0 && data.y >= 0.0);
        assert!(data.z.round() < 255.0 && data.z >= 0.0);
        return ChunkPos::new(data.x.round() as u8, data.y.round() as u8, data.z.round() as u8);
    }
}
impl ChunkVec1 {
    pub fn to_chunk_axis_idx(&self) -> u8 {
        let data = self.0;
        assert!(data.round() < 255.0 && data >= 0.0);
        return data.round() as u8;
    }
}