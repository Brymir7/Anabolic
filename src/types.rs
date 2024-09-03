use macroquad::math::Vec3;
#[derive(Clone, Copy, Debug  )]

pub struct ChunkVec3(pub Vec3);
#[derive(Debug  )]
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
        assert!(data.x.trunc() < 255.0 && data.x >= 0.0);
        assert!(data.y.trunc() < 255.0 && data.x >= 0.0);
        assert!(data.z.trunc() < 255.0 && data.x >= 0.0);
        return ChunkPos::new(data.x.trunc() as u8, data.z.trunc() as u8, data.y.trunc() as u8);
    }
}
pub struct Player {
    pub pos: ChunkVec3,
    pub vel: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}
#[derive(Clone, Copy, Debug)]
pub enum EntityType {
    SolidBlock,
    Player,
    None,
    InteractableBlock(u16),
}
enum InteractableBlockType {
    Portal,
    Trampoline,
}

pub struct InteractableBlocks {
    pub positions: Vec<Vec3>,
    pub b_type: InteractableBlockType,
}

