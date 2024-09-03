use config::{ window_conf, INITIAL_PLAYER_POS, LOOK_SPEED, PHYSICS_FRAME_TIME, WORLD_UP };
use macroquad::prelude::*;
use movement::MovementSystem;
pub mod movement;
pub mod config;

struct Player {
    pos: Vec3,
    vel: Vec3,
    yaw: f32,
    pitch: f32,
}

struct World {
    player: Player,
    camera: Camera3D,
    grabbed: bool,
}

impl World {
    fn default() -> Self {
        World {
            player: Player {
                pos: INITIAL_PLAYER_POS,
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
        }
    }
    fn update(&mut self) {
        MovementSystem::update_player(&mut self.player.pos, &mut self.player.vel);
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
                self.camera.position = self.player.pos;
                self.camera.up = up;
                self.camera.target = self.player.pos + front;
            }
        }
    }

    fn draw(&self) {
        set_camera(&self.camera);
        for x in 0..10 {
            for z in 0..10 {
                draw_cube_wires(vec3(x as f32, 0.0, z as f32), Vec3::splat(1.0), GREEN);
            }
        }
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
