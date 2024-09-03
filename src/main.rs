use std::cmp::{ max, min };

use config::{
    window_conf,
    INITIAL_PLAYER_POS,
    LOOK_SPEED,
    PHYSICS_FRAME_TIME,
    CHUNK_SIZE,
    WORLD_UP,
};
use macroquad::prelude::*;
use movement::MovementSystem;
use render::RenderSystem;
use types::{ ChunkVec3, EntityType, Player };
pub mod movement;
pub mod config;
pub mod render;
pub mod types;
struct World {
    player: Player,
    camera: Camera3D,
    world_layout: [[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    grabbed: bool,
}

impl World {
    fn default() -> Self {
        let mut world = World {
            player: Player {
                pos: ChunkVec3(INITIAL_PLAYER_POS),
                vel: Vec3::ZERO,
                yaw: 0.77,
                pitch: 0.0,
            },
            camera: Camera3D {
                position: INITIAL_PLAYER_POS + WORLD_UP,
                up: WORLD_UP,
                target: vec3(1.0, 1.8, 0.0),
                ..Default::default()
            },
            grabbed: true,
            //  X, Z, Y
            world_layout: [
                [[EntityType::None; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
                CHUNK_SIZE as usize
            ],
        };
        for x in 0..CHUNK_SIZE as usize {
            for z in 0..CHUNK_SIZE as usize {
                world.world_layout[x][z][0] = EntityType::SolidBlock;
            }
        }
        world
    }
    fn update(&mut self) {
        let surrounding_chunks = &self.get_chunk_of_layout(&self.player.pos.0, Vec3::splat(1.0));
        MovementSystem::update_player(&mut self.player.pos, &mut self.player.vel, surrounding_chunks);
    }

    fn get_chunk_of_layout(&self, center: &Vec3, radius: Vec3) -> Vec<Vec<Vec<EntityType>>> {
        let mut result = Vec::new();


        let min_x = max(0, (center.x - radius.x).floor() as usize);
        let min_z = max(0, (center.z - radius.z).floor() as usize);
        let min_y = max(0, (center.y - radius.y).floor() as usize);

        let max_x = min(CHUNK_SIZE - 1, (center.x + radius.x).ceil() as u8);
        let max_z = min(CHUNK_SIZE - 1, (center.z + radius.z).ceil() as u8);
        let max_y = min(CHUNK_SIZE - 1, (center.y + radius.y).ceil() as u8);


        for x in min_x..=max_x as usize {
            let mut zy_plane = Vec::new();
            for z in min_z..=max_z as usize {
                let mut y_line = Vec::new();
                for y in min_y..=max_y as usize {
                    y_line.push(self.world_layout[x][z][y].clone());
                }
                zy_plane.push(y_line);
            }
            result.push(zy_plane);
        }

        result
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.grabbed = !self.grabbed;
            set_cursor_grab(self.grabbed);
            show_mouse(!self.grabbed);
        }
        let delta = get_frame_time();
        if self.grabbed {
            let mouse_delta = mouse_delta_position();
            if self.grabbed {
                self.player.yaw += mouse_delta.x * delta * LOOK_SPEED;
                self.player.pitch += mouse_delta.y * delta * -LOOK_SPEED;
                self.player.pitch = self.player.pitch.clamp(-1.5, 1.5);
                let front = vec3(
                    self.player.yaw.cos() * self.player.pitch.cos(),
                    self.player.pitch.sin(),
                    self.player.yaw.sin() * self.player.pitch.cos()
                ).normalize();

                let right = front.cross(WORLD_UP).normalize();
                let up = right.cross(front).normalize();
                let mut player_vel = Vec3::ZERO;
                if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                    player_vel = front;
                }
                if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                    player_vel = -front;
                }
                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    player_vel = -right;
                }
                if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    player_vel = right;
                }
                self.player.vel = player_vel;
                self.camera.position = self.player.pos.0;
                self.camera.up = up;
                self.camera.target = self.player.pos.0 + front;
            }
        }
    }

    fn draw(&self) {
        set_camera(&self.camera);
        RenderSystem::render_world(&self.world_layout);
        set_default_camera()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut elapsed_time = 0.0;
    let mut world = World::default();
    set_cursor_grab(true);
    show_mouse(false);
    loop {
        clear_background(BLACK);
        elapsed_time += get_frame_time();
        world.handle_input();
        while elapsed_time >= PHYSICS_FRAME_TIME {
            world.update();
            elapsed_time = 0.0;
        }
        world.draw();
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 10.0, 20.0, WHITE);
        next_frame().await;
    }
}
