use miniquad::*;
use primitive_drawer::primitive_drawer::PrimitiveDrawer;
pub mod primitive_drawer;

struct Stage {
    ctx: Box<dyn RenderingBackend>,
    p_drawer: PrimitiveDrawer,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();
        let p_drawer = PrimitiveDrawer::default(&mut *ctx);
        Stage {
            ctx,
            p_drawer,
        }
    }
}
impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());
        self.p_drawer.draw_cube(0.0, 0.0, 0.0, 0.5, &mut *self.ctx);
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), move || Box::new(Stage::new()));
}
