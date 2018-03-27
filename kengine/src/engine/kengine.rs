use piston_window::*;
use opengl_graphics::OpenGL;
use engine::kgame::Kgame;
use assets::ksprite::Ksprite;
use engine::kevents::Kevents;

pub struct Kengine{
    window: PistonWindow,
    game: Kgame,
    ups: u64
}

impl Kengine{
    pub fn new(title: &str, width: u32, height: u32, ups: u64) -> Kengine{
        let opengl = OpenGL::V3_2;
        Kengine{
            window: WindowSettings::new(title,[width, height]).opengl(opengl).exit_on_esc(true).build().unwrap(),
            game: Kgame::new(opengl),
            ups: ups
        }
    }

    pub fn add_ksprite(&mut self, id: &str, k : Ksprite){
        self.game.a.add(String::from(id), k);
    }

    pub fn get_ksprite(&mut self, id: &str) -> &mut Ksprite{
        self.game.a.get_kassets().get_mut(id).unwrap()
    }

    pub fn run(&mut self) -> Kevents{
        let mut events = Events::new(EventSettings::new());
        events.set_ups(self.ups);
        while let Some(e) = events.next(&mut self.window){
            if let Some(r) = e.render_args(){
                self.game.render(&r);
            }

            if let Some(u) = e.update_args(){
                self.game.update(&u);
            }

            if let Some(i) = e.button_args(){
                return self.game.input(&i);
            }
        }
        return Kevents::new(None);
    }
}
