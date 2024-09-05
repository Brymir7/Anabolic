use shared::{
    config::{ CHUNK_SIZE, TILE_SIZE }, draw_cube_wires, types::{ AnimationState, ChunkVec3, EntityType, PossibleEnemySizes, RegularEnemies }, vec3, Color, Vec3, BLUE, GRAY, GREEN, RED, YELLOW // dont use macroquad types here, then avoid dependency and then we could make it compile quicker ?
};
pub mod animation;

pub trait Drawer {
    fn draw_cube_wires(&self, position: Vec3, size: Vec3, color: Color);
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
    positions: &Vec<ChunkVec3>,
    velocities: &Vec<Vec3>,
    animations: &Vec<AnimationState>,
    sizes: &Vec<PossibleEnemySizes>
) {
    for (i, enemy) in positions.iter().enumerate() {
        #[cfg(not(feature = "debug"))]
        render_default_enemy(
            screen,
            *enemy,
            velocities[i],
            sizes[i],
            animations[i].current_step,
            animations[i].max_step
        );
        #[cfg(feature = "debug")]
        render_default_enemy_with_hitbox(
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
    pos: ChunkVec3,
    vel: Vec3,
    size: PossibleEnemySizes,
    animation_step: f32,
    max_animation_step: f32
) {
    let scale = match size {
        PossibleEnemySizes::SMALL => Vec3::splat(0.25),
        PossibleEnemySizes::MEDIUM => Vec3::splat(0.5),
        PossibleEnemySizes::LARGE => Vec3::splat(0.75),
        PossibleEnemySizes::BOSS => Vec3::splat(1.25),
    };
    let is_x_dominant = vel.x < vel.z;
    let x_multiplier = is_x_dominant as u8;
    let z_multiplier = !is_x_dominant as u8;
    println!("{}", x_multiplier);
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
        pos + vec3(
            0.25 * (x_multiplier as f32) + leg_swing_offset * (z_multiplier as f32), 
            -0.75, 
            0.25 * (z_multiplier as f32) + leg_swing_offset * (x_multiplier as f32)
        ) * scale,
        Vec3::new(0.2, 0.5, 0.2) * scale,
        RED
    );
    
    // Left leg (moves backward)
    screen.drawer.draw_cube_wires(
        pos + vec3(
            -0.25 * (x_multiplier as f32) - leg_swing_offset * (z_multiplier as f32), 
            -0.75, 
            -0.25 * (z_multiplier as f32) - leg_swing_offset * (x_multiplier as f32)
        ) * scale,
        Vec3::new(0.2, 0.5, 0.2) * scale,
        RED);
}

#[no_mangle]
pub fn render_default_enemy_with_hitbox(
    screen: &Screen,
    pos: ChunkVec3,
    vel: Vec3,
    size: PossibleEnemySizes,
    animation_step: f32,
    max_animation_step: f32
) {
    let scale = RegularEnemies::get_vec3_size(size);
    let is_x_dominant = vel.x < vel.z;
    let x_multiplier = is_x_dominant as u8;
    let z_multiplier = !is_x_dominant as u8;
    println!("{}", x_multiplier);
    let pos = pos.0;
    // HITBOX 
    screen.drawer.draw_cube_wires(
        pos,
        RegularEnemies::get_hitbox_from_size(size),
        GRAY,
    );
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
        pos + vec3(
            0.25 * (x_multiplier as f32) + leg_swing_offset * (z_multiplier as f32), 
            -0.75, 
            0.25 * (z_multiplier as f32) + leg_swing_offset * (x_multiplier as f32)
        ) * scale,
        Vec3::new(0.2, 0.5, 0.2) * scale,
        RED
    );
    
    // Left leg (moves backward)
    screen.drawer.draw_cube_wires(
        pos + vec3(
            -0.25 * (x_multiplier as f32) - leg_swing_offset * (z_multiplier as f32), 
            -0.75, 
            -0.25 * (z_multiplier as f32) - leg_swing_offset * (x_multiplier as f32)
        ) * scale,
        Vec3::new(0.2, 0.5, 0.2) * scale,
        RED);

    
}
