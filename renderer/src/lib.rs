use shared::{
    config::{ CHUNK_SIZE, TILE_SIZE },
    draw_cube_wires,
    types::EntityType,
    vec3,
    Color,
    Vec3,
    GREEN,
    RED,
};
pub trait Drawer {
    fn draw_cube_wires(&self, position: Vec3, size: Vec3, color: Color);
}
pub struct Screen {
    pub drawer: Box<dyn Drawer>,
}

#[no_mangle]
pub fn render_world(
    screen: &Screen,
    world_layout: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
) {
    for x in 0..world_layout.len() {
        for z in 0..world_layout.len() {
            for y in 0..world_layout.len() {
                match world_layout[x][z][y] {
                    EntityType::SolidBlock => {
                        screen.drawer.draw_cube_wires(
                            vec3(x as f32, y as f32, z as f32),
                            Vec3::splat(TILE_SIZE),
                            GREEN
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}
#[no_mangle]
pub fn render_default_enemy(screen:&Screen, pos: Vec3, scale: Vec3) {
    // HEAD
    screen.drawer.draw_cube_wires(pos, Vec3::splat(2.0), RED);
}
