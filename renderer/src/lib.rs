use std::{
    collections::{ HashMap, VecDeque },
    f32::consts::{ FRAC_PI_2, FRAC_PI_4, FRAC_PI_8, PI },
};

use shared::{
    config::{ SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE },
    types::{
        AnimationState, ChunkVec3, Enemies, EnemyType, PossibleEnemySizes, Textures, VoxelMesh, WeaponType
    },
    vec2,
    vec3,
    Color,
    DrawRectangleParams,
    DrawTextureParams,
    Vec2,
    Vec3,
    GRAY,
    GREEN,
    RED,
    WHITE,
    YELLOW, // dont use macroquad types here, then avoid dependency and then we could make it compile quicker ?
};
pub mod animation;
pub mod debug;
pub trait Drawer {
    fn draw_cube_wires(&self, position: Vec3, size: Vec3, color: Color);
    fn draw_rectangle(&self, position: Vec2, width: f32, height: f32, color: Color);
    fn draw_rectangle_lines_ex(
        &self,
        position: Vec2,
        width: f32,
        height: f32,
        params: DrawRectangleParams
    );
    fn draw_rectangle_lines(&self, position: Vec2, width: f32, height: f32, color: Color);
    fn draw_triangle(&self, pos1: Vec2, pos2: Vec2, pos3: Vec2, color: Color);
    fn draw_circle_lines(&self, position: Vec2, radius: f32, color: Color);
    // fn draw_texture_ex(
    //     &self,
    //     texture: &Textures,
    //     x: f32,
    //     y: f32,
    //     color: Color,
    //     params: DrawTextureParams
    // );
    fn draw_voxel_mesh(&self, mesh: &VoxelMesh);
}
pub struct Screen {
    pub drawer: Box<dyn Drawer>,
}

#[no_mangle]
pub fn render_solid_blocks(screen: &Screen, positions: &Vec<ChunkVec3>) {
    for pos in positions {
        let pos = pos.0;
        screen.drawer.draw_cube_wires(vec3(pos.x, pos.y, pos.z), Vec3::splat(TILE_SIZE), GREEN);
    }
}
#[no_mangle]
pub fn render_regular_enemies(
    screen: &Screen,
    enemies: &Enemies,
) {
    for (i, enemy) in enemies.positions.iter().enumerate() {
        #[cfg(not(feature = "debug"))]
        render_default_enemy(
            screen,
            enemies.e_type[i],
            *enemy,
            enemies.velocities[i],
            enemies.size[i],
            enemies.animation_state[i].current_step,
            enemies.animation_state[i].max_step
        );
        #[cfg(feature = "debug")]
        render_default_enemy_with_hitbox(
            screen,
            enemies.e_type[i],
            *enemy,
            enemies.velocities[i],
            enemies.size[i],
            enemies.animation_state[i].current_step,
            enemies.animation_state[i].max_step
        );
    }
}
#[no_mangle]
pub fn render_flying_enemies(
    screen: &Screen,
    positions: &Vec<ChunkVec3>,
    velocities: &Vec<Vec3>,
    animations: &Vec<AnimationState>,
    sizes: &Vec<PossibleEnemySizes>
) {
    for (i, enemy) in positions.iter().enumerate() {
        #[cfg(not(feature = "debug"))]
        render_flying_enemy(
            screen,
            *enemy,
            velocities[i],
            sizes[i],
            animations[i].current_step,
            animations[i].max_step
        );
        #[cfg(feature = "debug")]
        render_flying_enemy_with_hitbox(
            screen,
            *enemy,
            velocities[i],
            sizes[i],
            animations[i].current_step,
            animations[i].max_step
        );
    }
}
#[no_mangle]
pub fn render_default_enemy(
    screen: &Screen,
    e_type: EnemyType,
    pos: ChunkVec3,
    vel: Vec3,
    size: PossibleEnemySizes,
    animation_step: f32,
    max_animation_step: f32
) {
    if e_type == EnemyType::Empty {
        return;
    }
    let scale = match size {
        PossibleEnemySizes::SMALL => Vec3::splat(0.25),
        PossibleEnemySizes::MEDIUM => Vec3::splat(0.5),
        PossibleEnemySizes::LARGE => Vec3::splat(0.75),
        PossibleEnemySizes::BOSS => Vec3::splat(1.25),
    };
    let is_x_dominant = vel.x < vel.z;
    let x_multiplier = is_x_dominant as u8;
    let z_multiplier = !is_x_dominant as u8;

    let pos = pos.0;

    // HEAD
    screen.drawer.draw_cube_wires(
        pos + vec3(0.0, 0.75, 0.0) * scale,
        Vec3::splat(0.5) * scale,
        RED
    );

    // EYES
    screen.drawer.draw_cube_wires(
        pos + vec3(0.15 * (x_multiplier as f32), 0.9, 0.15 * (z_multiplier as f32)) * scale,
        Vec3::splat(0.1) * scale,
        YELLOW
    );
    screen.drawer.draw_cube_wires(
        pos + vec3(-0.15 * (x_multiplier as f32), 0.9, -0.15 * (z_multiplier as f32)) * scale,
        Vec3::splat(0.1) * scale,
        YELLOW
    );

    // BODY
    screen.drawer.draw_cube_wires(pos, Vec3::splat(1.0) * scale, RED);
    screen.drawer.draw_cube_wires(pos, Vec3::splat(0.5) * scale, YELLOW);

    // LEGS
    let animation_phase = (animation_step / max_animation_step) * std::f32::consts::PI * 2.0;
    let leg_swing_offset = vel.length() * animation_phase.sin() * scale.x; // Forward/backward movement based on size and step

    // Right leg (moves forward)
    screen.drawer.draw_cube_wires(
        pos +
            vec3(
                0.25 * (x_multiplier as f32) + leg_swing_offset * (z_multiplier as f32),
                -0.75,
                0.25 * (z_multiplier as f32) + leg_swing_offset * (x_multiplier as f32)
            ) *
                scale,
        Vec3::new(0.2, 0.5, 0.2) * scale,
        RED
    );

    // Left leg (moves backward)
    screen.drawer.draw_cube_wires(
        pos +
            vec3(
                -0.25 * (x_multiplier as f32) - leg_swing_offset * (z_multiplier as f32),
                -0.75,
                -0.25 * (z_multiplier as f32) - leg_swing_offset * (x_multiplier as f32)
            ) *
                scale,
        Vec3::new(0.2, 0.5, 0.2) * scale,
        RED
    );
}

#[no_mangle]
pub fn render_default_enemy_with_hitbox(
    screen: &Screen,
    e_type: EnemyType,
    pos: ChunkVec3,
    vel: Vec3,
    size: PossibleEnemySizes,
    animation_step: f32,
    max_animation_step: f32
) {
    if e_type == EnemyType::Empty {
        return;
    }
    let scale = Enemies::get_vec3_size(size);
    let is_x_dominant = vel.x.abs() < vel.z.abs();
    let x_multiplier = is_x_dominant as u8;
    let z_multiplier = !is_x_dominant as u8;

    let pos = pos.0;
    // HITBOX
    screen.drawer.draw_cube_wires(pos, Enemies::get_hitbox_from_size(size), GRAY);
    // HEAD
    screen.drawer.draw_cube_wires(
        pos + vec3(0.0, 0.75, 0.0) * scale,
        Vec3::splat(0.5) * scale,
        RED
    );

    // EYES
    screen.drawer.draw_cube_wires(
        pos + vec3(0.15 * (x_multiplier as f32), 0.9, 0.15 * (z_multiplier as f32)) * scale,
        Vec3::splat(0.1) * scale,
        YELLOW
    );
    screen.drawer.draw_cube_wires(
        pos + vec3(-0.15 * (x_multiplier as f32), 0.9, -0.15 * (z_multiplier as f32)) * scale,
        Vec3::splat(0.1) * scale,
        YELLOW
    );

    // BODY
    screen.drawer.draw_cube_wires(pos, Vec3::splat(1.0) * scale, RED);
    screen.drawer.draw_cube_wires(pos, Vec3::splat(0.5) * scale, YELLOW);

    // LEGS
    let animation_phase = (animation_step / max_animation_step) * std::f32::consts::PI * 2.0;
    let leg_swing_offset = vel.length() * animation_phase.sin() * scale.x; // Forward/backward movement based on size and step

    // Right leg (moves forward)
    screen.drawer.draw_cube_wires(
        pos +
            vec3(
                0.25 * (x_multiplier as f32) + leg_swing_offset * (z_multiplier as f32),
                -0.75,
                0.25 * (z_multiplier as f32) + leg_swing_offset * (x_multiplier as f32)
            ) *
                scale,
        Vec3::new(0.2, 0.5, 0.2) * scale,
        RED
    );

    // Left leg (moves backward)
    screen.drawer.draw_cube_wires(
        pos +
            vec3(
                -0.25 * (x_multiplier as f32) - leg_swing_offset * (z_multiplier as f32),
                -0.75,
                -0.25 * (z_multiplier as f32) - leg_swing_offset * (x_multiplier as f32)
            ) *
                scale,
        Vec3::new(0.2, 0.5, 0.2) * scale,
        RED
    );
}
#[no_mangle]
pub fn render_flying_enemy(
    screen: &Screen,
    pos: ChunkVec3,
    vel: Vec3,
    size: PossibleEnemySizes,
    animation_step: f32,
    max_animation_step: f32
) {
    let scale = Enemies::get_vec3_size(size);
    let pos = pos.0;
    let size_animation = (animation_step * PI).sin() * 0.5;
    // BODY
    screen.drawer.draw_cube_wires(pos, Vec3::splat(1.0) * scale * size_animation, RED);
    screen.drawer.draw_cube_wires(pos, Vec3::splat(0.5) * scale * size_animation, YELLOW);
}

#[no_mangle]
pub fn render_flying_enemy_with_hitbox(
    screen: &Screen,
    pos: ChunkVec3,
    vel: Vec3,
    size: PossibleEnemySizes,
    animation_step: f32,
    max_animation_step: f32
) {
    let scale = Enemies::get_vec3_size(size);
    let pos = pos.0;
    let size_animation = ((animation_step * PI).sin() * 2.0).max(0.5);
    // HITBOX
    screen.drawer.draw_cube_wires(pos, Enemies::get_hitbox_from_size(size), GRAY);

    screen.drawer.draw_cube_wires(pos, Vec3::splat(1.0) * scale * size_animation, RED);
    screen.drawer.draw_cube_wires(pos, Vec3::splat(0.5) * scale * size_animation, YELLOW);
}

#[no_mangle]
pub fn render_player_pov(
    screen: &Screen,
    voxel_mesh: &VoxelMesh,
    w_type: WeaponType,
    animation_state: &AnimationState
) {
    const SCREEN_X_OFFSET: f32 = (SCREEN_WIDTH as f32) / 2.0;
    const SCREEN_Y_OFFSET: f32 = (SCREEN_HEIGHT as f32) / 2.0;
    let bobbing = (animation_state.current_step * PI).sin() * 0.25;
    // CROSSHAIR
    screen.drawer.draw_circle_lines(vec2(SCREEN_X_OFFSET, SCREEN_Y_OFFSET), 5.0, WHITE);
    match w_type {
        WeaponType::Shotgun => {
                screen.drawer.draw_voxel_mesh(voxel_mesh);
        }
    }
}
