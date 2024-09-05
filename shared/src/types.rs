use macroquad::math::Vec3;
#[derive(Clone, Copy, Debug)]
pub struct ChunkVec3(pub Vec3);
#[derive(Debug)]
pub struct ChunkPos {
    pub x: u8,
    pub z: u8,
    pub y: u8,
}
impl ChunkPos {
    fn new(x: u8, z: u8, y: u8) -> Self {
        ChunkPos {
            x,
            z,
            y,
        }
    }
}
impl ChunkVec3 {
    pub fn to_chunk(&self) -> ChunkPos {
        let data = self.0;
        assert!(data.x.round() < 255.0 && data.x >= 0.0);
        assert!(data.y.round() < 255.0 && data.y >= 0.0);
        assert!(data.z.round() < 255.0 && data.z >= 0.0);
        return ChunkPos::new(data.x.round() as u8, data.z.round() as u8, data.y.round() as u8);
    }
}
pub struct Player {
    pub pos: ChunkVec3,
    pub vel: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct EnemyHandle(pub u16);

#[derive(Clone, Copy, Debug)]
pub enum EntityType {
    SolidBlock,
    Player,
    None,
    InteractableBlock(u16),
    RegularEnemy(EnemyHandle),
    FlyingEnemy(EnemyHandle),
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
pub struct FlyingEnemies {
    pub positions: Vec<ChunkVec3>,
    pub velocities: Vec<Vec3>,
    pub animation_state: Vec<AnimationState>,
    pub size: Vec<PossibleEnemySizes>,
}
pub struct RegularEnemies {
    pub positions: Vec<ChunkVec3>,
    pub velocities: Vec<Vec3>,
    pub animation_state: Vec<AnimationState>,
    pub size: Vec<PossibleEnemySizes>,
}

pub struct SolidBlocks {
    pub positions: Vec<ChunkVec3>,
}