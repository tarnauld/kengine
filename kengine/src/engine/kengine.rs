use piston_window::*;
use opengl_graphics::OpenGL;
use engine::kgame::Kgame;

pub struct Kengine {
    window: PistonWindow,
    game: Kgame
}

impl Kengine{
    pub fn new(title: String, width: u32, height: u32) -> Kengine{
        let opengl = OpenGL::V3_2;
        Kengine{
            window: WindowSettings::new(title,[width, height]).opengl(opengl).exit_on_esc(true).build().unwrap(),
            game: Kgame::new()
        }
    }

    pub fn init_render(&mut self, f: &Fn() -> i32){

    }

    pub fn init_update(&mut self, f: &Fn() -> i32){

    }

    pub fn run(&mut self){
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window){
            if let Some(r) = e.render_args(){
                self.game.render(&r);
            }

            if let Some(u) = e.update_args(){
                self.game.update(&u);
            }

            if let Some(i) = e.button_args(){
                self.game.input(&i);
            }
        }
    }
}
