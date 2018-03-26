use assets::kassets::Kassets;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use graphics::*;
use input::keyboard::Keyboard;

pub struct Kgame{
    gl: GlGraphics,
    pub k: Option<Keyboard>,
    pub a: Kassets
}

impl Kgame{
    pub fn new(opengl: OpenGL) -> Kgame{
        Kgame{
            gl: GlGraphics::new(opengl),
            k: None,
            a: Kassets::new()
        }
    }

    pub fn render(&mut self, args: &RenderArgs){
        let k = self.a.get_kassets();

        self.gl.draw(args.viewport(), |c, gl| {
            for o in k.iter(){
                let (texture, x, y) = o.get_ksprite();
                image(texture.as_ref().unwrap().get_texture(), c.transform.trans(x, y), gl);
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs){
        for o in self.a.get_kassets().iter_mut(){
            o.move_ksprite();
        }
    }

    pub fn input(&self, _inp: &ButtonArgs){

    }
}
