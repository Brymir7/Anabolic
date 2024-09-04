use std::cmp::{ max, min };

use config::{
    window_conf, CHUNK_SIZE, INITIAL_PLAYER_POS, JUMP_STRENGTH, LOOK_SPEED, PHYSICS_FRAME_TIME, WORLD_UP
};
use macroquad::prelude::*;
use movement::MovementSystem;
use render::RenderSystem;
use types::{ ChunkVec3, EntityType, Player };
use util::vec3_no_y;
pub mod movement;
pub mod config;
pub mod render;
pub mod types;
pub mod util;
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
        println!("{:?}", world.world_layout.len());
        for x in 0..CHUNK_SIZE as usize {
            for z in 0..CHUNK_SIZE as usize {
                world.world_layout[x][z][0] = EntityType::SolidBlock;
            }
        }
        world
    }
    fn update(&mut self) {
        MovementSystem::update_player(&mut self.player.pos, &mut self.player.vel, &self.world_layout);
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
                    player_vel = vec3_no_y(front);
                }
                if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                    player_vel = -vec3_no_y(front);
                }
                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    player_vel = -vec3_no_y(right);
                }
                if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    player_vel = vec3_no_y(right);
                }
                if is_key_down(KeyCode::Space) {
                    player_vel.y = JUMP_STRENGTH;
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
        draw_cube_wires(self.player.pos.0, Vec3::new(1.0, 2.0, 1.0), RED);
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
