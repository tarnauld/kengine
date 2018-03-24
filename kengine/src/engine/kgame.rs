use assets::kassets::Kassets;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use graphics::*;

pub struct Kgame {
    gl: GlGraphics,
    pub a: Kassets
}

impl Kgame{
    pub fn new(opengl: OpenGL) -> Kgame{
        Kgame{
            gl: GlGraphics::new(opengl),
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

    pub fn update(&self, _args: &UpdateArgs){

    }

    pub fn input(&self, _inp: &ButtonArgs){

    }
}
