use shared::{
    config::{ CHUNK_SIZE, TILE_SIZE },
    draw_cube_wires,
    types::EntityType,
    vec3,
    Vec3,
    GREEN,
    RED,
};
pub trait Drawer {
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32);
}
pub struct Screen {
    pub drawer: Box<dyn Drawer>,
}

#[no_mangle]
pub fn render_world(
    world_layout: &[[[EntityType; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]
) {
    for x in 0..world_layout.len() {
        for z in 0..world_layout.len() {
            for y in 0..world_layout.len() {
                match world_layout[x][z][y] {
                    EntityType::SolidBlock => {
                        draw_cube_wires(
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
pub fn render_default_enemy(pos: Vec3, scale: Vec3) {
    // HEAD
    draw_cube_wires(pos, Vec3::splat(2.0), GREEN);
}


#[no_mangle]
pub fn render_text(screen: &Screen) {
    // HEAD
    screen.drawer.draw_text("d asdasdsaas", 120.0, 120.0, 40.0);

}


