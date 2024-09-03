use macroquad::{math::{vec3, Vec3}, window::Conf};
pub const PHYSICS_FRAMES_PER_SECOND: f32 = 60.0;
pub const PHYSICS_FRAME_TIME: f32 = 1.0 / 60.0;
pub const MOVE_SPEED: f32 = 5.0;
pub const LOOK_SPEED: f32 = 40.0;
pub const WORLD_UP: Vec3 = vec3(0.0, 1.0, 0.0);
pub const INITIAL_PLAYER_POS: Vec3 = vec3(0.0, 1.0, 0.0);
pub const CHUNK_SIZE: u8 = 16;
pub const MAX_BLOCK_TYPES: u8 = u8::MAX;
pub const GRAVITY: f32 = -9.81;
pub const JUMP_STRENGTH: f32 = 10.0;
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Anabolic".to_owned(),
        window_width: 1920,
        window_height: 1080,
        window_resizable: false,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        ..Default::default()
    }
}
