use std::ops::Add;

use macroquad::math::Vec3;

use crate::types::{AnimationCallbackEvent, AnimationState, ChunkVec3, EnemyHandle, FlyingEnemies, PossibleEnemySizes, RegularEnemies, SolidBlocks};

impl RegularEnemies {
    pub fn new() -> Self {
        RegularEnemies {
            positions: Vec::new(),
            velocities: Vec::new(),
            animation_state: Vec::new(),
            size: Vec::new(),
        }
    }
    pub fn new_enemy(&mut self, pos: ChunkVec3, vel: Vec3, size: PossibleEnemySizes) -> EnemyHandle {
        self.positions.push(pos);
        self.velocities.push(vel);
        self.animation_state.push(AnimationState::default());
        self.size.push(size);
        return EnemyHandle((self.positions.len() - 1) as u16)
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
    pub fn new_enemy(&mut self, pos: ChunkVec3, vel: Vec3, size: PossibleEnemySizes) -> EnemyHandle {
        self.positions.push(pos);
        self.velocities.push(vel);
        self.animation_state.push(AnimationState::default());
        self.size.push(size);
        return EnemyHandle((self.positions.len() - 1) as u16)
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