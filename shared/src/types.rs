use macroquad::{color::Color, math::Vec3};
#[derive(Clone, Copy, Debug)]
pub struct ChunkVec3(pub Vec3);

#[derive(Debug, Clone, Copy)]
pub struct ChunkPos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Clone, Copy)]
pub enum WeaponType {
    Shotgun,
    // ChainLightning,
    // Sniper,
    // Pistol,
}
pub struct Weapon {
    pub damage: f32,
    pub reload_speed: f32,
    pub time_last_reload: f32,
    pub w_type: WeaponType,
}
pub struct CurrWeapon(pub usize);
pub struct MaxWeapon(pub usize);
pub struct Player {
    pub pos: ChunkVec3,
    pub vel: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub weapon_unlocked: MaxWeapon,
    pub weapons: [Weapon; 1],
    pub curr_weapon: CurrWeapon,
    pub animation_state: AnimationState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnemyHandle(pub u16);


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntityType {
    SolidBlock,
    Player,
    InteractableBlock(u16),
    Enemy(EnemyHandle),
}
enum InteractableBlockType {
    Portal,
    Trampoline,
}

pub struct InteractableBlocks {
    pub positions: Vec<Vec3>,
    pub b_type: InteractableBlockType,
}
#[derive(PartialEq, Clone, Copy)]
pub enum AnimationCallbackEvent {
    KillEnemy,
    None
}
pub struct AnimationState {
    pub current_step: f32,
    pub max_step: f32,
    pub callback: AnimationCallbackEvent,
}

#[derive(Clone, Copy)]
pub enum PossibleEnemySizes {
    SMALL,
    MEDIUM,
    LARGE,
    BOSS,
} 
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum EnemyType {
    Flying,
    Regular,
    Cube,
    Empty,
}

pub struct Enemies {
    pub positions: Vec<ChunkVec3>,
    pub velocities: Vec<Vec3>,
    pub animation_state: Vec<AnimationState>,
    pub size: Vec<PossibleEnemySizes>,
    pub healths: Vec<u8>,
    pub e_type: Vec<EnemyType>
}

pub struct SolidBlocks {
    pub positions: Vec<ChunkVec3>,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Textures {
    Pistol,
}


#[derive(Debug, Clone, Copy)]
pub enum WorldEvent {
    KillEnemy(EnemyHandle),
    HitEnemy(EnemyHandle),
}

#[derive(Clone, Copy)]
pub struct Voxel {
    pub position: Vec3,
    pub color: Color,
}

pub struct VoxelMesh {
    pub voxels: Vec<Voxel>,
}