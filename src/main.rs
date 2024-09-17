use std::{ collections::{ HashMap, VecDeque }, fs::DirEntry, process::exit, time::Duration };
use shared::{ config::window_conf, types::{ ChunkPos, Enemies, EnemyHandle } };
use macroquad::prelude::*;
use movement::MovementSystem;
use shared::{
    config::{
        CHUNK_SIZE,
        INITIAL_PLAYER_POS,
        JUMP_STRENGTH,
        LOOK_SPEED,
        PHYSICS_FRAME_TIME,
        WORLD_UP,
    },
    types::{ ChunkVec3, EntityType, Player, SolidBlocks, Textures, WorldEvent, VoxelMesh },
    Lazy,
};
use shooting::shoot;
use spawning::{ update_spawning_system, SpawningSystem };
use util::{ load_and_convert_texture, load_voxel_data, vec3_no_y };
pub mod movement;
pub mod util;
pub mod spawning;
pub mod shooting;

use render::{ Drawer, Screen };
static TEXTURE_TO_VOXEL_MESH: Lazy<HashMap<Textures, VoxelMesh>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(Textures::Pistol, load_voxel_data("textures/pistol.vox"));
    map
});
#[hot_lib_reloader::hot_module(dylib = "render")]
mod hot_r_renderer {
    hot_functions_from_file!("renderer/src/lib.rs");
    hot_functions_from_file!("renderer/src/animation.rs");
    hot_functions_from_file!("renderer/src/debug.rs");
    use render::Screen;
    use shared::{
        config::CHUNK_SIZE,
        types::{
            ChunkVec3,
            EntityType,
            AnimationState,
            PossibleEnemySizes,
            AnimationCallbackEvent,
            WeaponType,
            Enemies,
            VoxelMesh,
            EnemyType,
        },
        Vec3,
    };
}
pub struct World {
    player: Player,
    camera: Camera3D,
    enemies: Enemies,
    solid_blocks: SolidBlocks,
    pub world_layout: [
        [[Vec<EntityType>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
        CHUNK_SIZE as usize
    ],
    grabbed: bool,
    world_event_queue: VecDeque<WorldEvent>,
}

impl World {
    fn default() -> Self {
        let world_layout: [
            [[Vec<EntityType>; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];
            CHUNK_SIZE as usize
        ] = core::array::from_fn(|_| {
            core::array::from_fn(|_| { core::array::from_fn(|_| { Vec::new() }) })
        });
        let mut world = World {
            player: Player::default(),
            camera: Camera3D {
                position: INITIAL_PLAYER_POS,
                up: WORLD_UP,
                target: vec3(1.0, 1.8, 0.0),
                ..Default::default()
            },
            grabbed: true,
            world_layout: world_layout,
            enemies: Enemies::new(),
            solid_blocks: SolidBlocks::new(), // make static
            world_event_queue: VecDeque::new(),
        };
        world.world_layout[INITIAL_PLAYER_POS.x as usize][INITIAL_PLAYER_POS.y as usize][
            INITIAL_PLAYER_POS.z as usize
        ].push(EntityType::Player);

        for x in 0..CHUNK_SIZE as usize {
            for z in 0..CHUNK_SIZE as usize {
                world.world_layout[x][0][z].push(EntityType::SolidBlock);
                world.solid_blocks.new_block(ChunkVec3(vec3(x as f32, 0.0, z as f32)));
            }
        }
        world.world_layout[3][8][3].push(
            EntityType::Enemy(
                world.enemies.new_enemy(
                    ChunkVec3(vec3(3.0, 8.0, 3.0)),
                    vec3(1.0, 0.0, 0.0),
                    shared::types::PossibleEnemySizes::SMALL,
                    1,
                    shared::types::EnemyType::Regular
                )
            )
        );
        world.world_layout[12][8][3].push(
            EntityType::Enemy(
                world.enemies.new_enemy(
                    ChunkVec3(vec3(12.0, 8.0, 3.0)),
                    vec3(-1.0, 0.0, 0.0),
                    shared::types::PossibleEnemySizes::SMALL,
                    1,
                    shared::types::EnemyType::Regular
                )
            )
        );
        world
    }

    fn remove_enemy(&mut self, h: EnemyHandle) {
        let index = h.0 as usize;
        if index < self.enemies.positions.len() {
            let position = self.enemies.positions[index];
            let size = self.enemies.size[index];
            let hitbox = Enemies::get_hitbox_from_size(size);

            let occupied_tiles = Enemies::get_occupied_tiles(&position, &(hitbox * 0.5));
            println!("occupied {:?}", occupied_tiles);
            for tile in occupied_tiles {
                self.world_layout[tile.x as usize][tile.y as usize][tile.z as usize].retain(
                    |entity| {
                        match entity {
                            EntityType::Enemy(h) => { h != h }
                            _ => { true }
                        }
                    }
                );
            }
            for (x, x_layer) in self.world_layout.iter().enumerate() {
                for (y, y_layer) in x_layer.iter().enumerate() {
                    for (z, entities) in y_layer.iter().enumerate() {
                        for entity in entities {
                            if let EntityType::Enemy(eh) = entity {
                                if *eh == h {
                                    println!(
                                        "Enemy found at position: x: {}, y: {}, z: {}",
                                        x,
                                        y,
                                        z
                                    );
                                }
                            }
                        }
                    }
                }
            }
            self.enemies.remove_enemy(h);
        }
    }

    fn handle_world_events(&mut self) {
        while let Some(event) = self.world_event_queue.pop_front() {
            println!("event {:?}", event);
            match event {
                WorldEvent::KillEnemy(h) => {
                    self.remove_enemy(h);
                }
                WorldEvent::HitEnemy(h) => {
                    let index = h.0 as usize;
                    if index < self.enemies.healths.len() {
                        if self.enemies.healths[index] > 1 {
                            self.enemies.healths[index] -= 1;
                        } else {
                            self.world_event_queue.push_back(WorldEvent::KillEnemy(h));
                        }
                    }
                }
            }
        }
    }
    fn update(&mut self, spawner: &mut SpawningSystem) {
        self.handle_world_events();
        let player_chunk = self.player.pos.to_chunk();
        MovementSystem::update_player(
            &mut self.player.pos,
            &mut self.player.vel,
            &self.enemies,
            &mut self.world_layout
        );
        MovementSystem::update_enemies(&self.player.pos, &mut self.enemies, &mut self.world_layout);
        update_spawning_system(self, spawner, Duration::from_secs_f32(PHYSICS_FRAME_TIME));
        debug_assert!(
            self.world_layout[player_chunk.x as usize][player_chunk.y as usize][
                player_chunk.z as usize
            ].contains(&EntityType::Player)
        );
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.grabbed = !self.grabbed;
            set_cursor_grab(self.grabbed);
            show_mouse(!self.grabbed);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            self.grabbed = true;
            set_cursor_grab(self.grabbed);
            show_mouse(!self.grabbed);
            self.world_event_queue.extend(
                shoot(&mut self.player, &self.enemies, &self.world_layout)
            );
        }
        if is_key_pressed(KeyCode::E) {
            self.player.swap_next_weapon();
        }

        let delta = get_frame_time();
        if self.grabbed {
            let mouse_delta = mouse_delta_position();

            self.player.yaw -= mouse_delta.x * delta * LOOK_SPEED;
            self.player.pitch -= mouse_delta.y * delta * -LOOK_SPEED;
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
            self.camera.target = self.camera.position + front;
        }
        if is_key_down(KeyCode::V) {
            exit(0);
        }
        if is_key_down(KeyCode::R) {
            self.world_layout = World::default().world_layout;
            self.enemies = World::default().enemies;
        }

        if is_key_down(KeyCode::F) {
            println!(
                "player pos as chunk {:?}, non chunk {:?}",
                self.player.pos.to_chunk(),
                self.player.pos
            );
            println!(
                "Enemies {:?}",
                self.enemies.positions.iter().map(|f| f.to_chunk())
            );
        }
    }

    #[cfg(not(feature = "hot-reload"))]
    fn draw(&self) {
        set_camera(&self.camera);
        // draw_cube_wires(self.player.pos.0, Vec3::new(1.0, 2.0, 1.0), RED);
        // hot_r_renderer::render_world(&self.world_layout);
        // hot_r_renderer::render_default_enemy(vec3(5.0, 1.0, 5.0), Vec3::splat(1.0));
        set_default_camera()
    }

    #[cfg(feature = "hot-reload")]
    fn draw(&mut self, screen: &Screen) {
        // needs to be mutable because of animation states, maybe refactor into a world separate struct?

        use shared::config::SCREEN_WIDTH;
        set_camera(&self.camera);
        hot_r_renderer::update_animations(&mut self.enemies.animation_state, get_frame_time());
        hot_r_renderer::update_animation(&mut self.player.animation_state, get_frame_time());
        hot_r_renderer::render_solid_blocks(screen, &self.solid_blocks.positions);
        hot_r_renderer::render_regular_enemies(screen, &self.enemies);

        hot_r_renderer::render_enemy_world_positions(screen, &self.world_layout, &self.enemies);
        set_default_camera();
        draw_text(
            &format!("Enemies: {}", self.enemies.e_type.len()),
            SCREEN_WIDTH as f32 - 150.0, // X position (top left corner)
            20.0, // Y position (top left corner)
            30.0, // Font size
            WHITE, // Color
        );
        let weapon_mesh = TEXTURE_TO_VOXEL_MESH.get(&Textures::Pistol).expect(
            "Failed to load weapon"
        );
        hot_r_renderer::render_player_pov(
            screen,
            weapon_mesh,
            self.player.get_current_weapon().w_type,
            &self.player.animation_state
        );
    }
}

pub struct DrawerImpl;
impl Drawer for DrawerImpl {
    fn draw_cube_wires(&self, position: Vec3, size: Vec3, color: Color) {
        macroquad::prelude::draw_cube_wires(position, size, color);
    }
    fn draw_rectangle(&self, position: Vec2, width: f32, height: f32, color: Color) {
        macroquad::prelude::draw_rectangle(position.x, position.y, width, height, color);
    }
    fn draw_triangle(&self, position1: Vec2, position2: Vec2, position3: Vec2, color: Color) {
        macroquad::prelude::draw_triangle(position1, position2, position3, color);
    }
    fn draw_rectangle_lines_ex(
        &self,
        position: Vec2,
        width: f32,
        height: f32,
        params: DrawRectangleParams
    ) {
        macroquad::prelude::draw_rectangle_lines_ex(
            position.x,
            position.y,
            width,
            height,
            1.0,
            params
        );
    }
    fn draw_rectangle_lines(&self, position: Vec2, width: f32, height: f32, color: Color) {
        macroquad::prelude::draw_rectangle_lines(position.x, position.y, width, height, 1.0, color);
    }
    fn draw_circle_lines(&self, position: Vec2, radius: f32, color: Color) {
        macroquad::prelude::draw_circle_lines(position.x, position.y, radius, 1.0, color);
    }

    fn draw_voxel_mesh(&self, mesh: &VoxelMesh) {
        set_camera(
            &(Camera3D { // aligns the weapon to the bottom left of the screen
                position: vec3(-3.35, 5.35, -12.5),
                target: vec3(7.0, 5.0, 0.0),
                up: vec3(0.0, 1.0, 0.0),
                ..Default::default()
            })
        );
        for voxel in &mesh.voxels {
            draw_cube(voxel.position, vec3(1.0, 1.0, 1.0) * 0.5, None, voxel.color);
        }
        set_default_camera()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut elapsed_time = 0.0;
    let mut world = World::default();
    let mut spawning_sys = SpawningSystem::new();
    #[cfg(feature = "hot-reload")]
    let drawer: Box<dyn Drawer> = Box::new(DrawerImpl {});
    #[cfg(feature = "hot-reload")]
    let screen = Screen { drawer };
    set_cursor_grab(true);
    show_mouse(false);

    loop {
        clear_background(BLACK);
        elapsed_time += get_frame_time();
        world.handle_input();
        while elapsed_time >= PHYSICS_FRAME_TIME {
            world.update(&mut spawning_sys);
            elapsed_time = 0.0;
        }
        #[cfg(feature = "hot-reload")]
        world.draw(&screen);
        #[cfg(not(feature = "hot-reload"))]
        world.draw();

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 10.0, 20.0, WHITE);
        next_frame().await;
    }
}
