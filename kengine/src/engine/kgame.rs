use assets::kassets::Kassets;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use graphics::*;
use input::keys::convert_key;
use engine::kevents::Kevents;

pub struct Kgame{
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
            for (_i, o) in k{
                let (texture, x, y) = o.get_ksprite();
                image(texture.as_ref().unwrap().get_texture(), c.transform.trans(x, y), gl);
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs){
        for (_i, o) in self.a.get_kassets(){
            o.move_ksprite();
        }
    }

    pub fn input(&mut self, inp: &ButtonArgs) -> Kevents{
        let b = inp.button;

        if inp.state == ButtonState::Press{
            Kevents::new(convert_key(&b))
        }else{
            Kevents::new(None)
        }
    }
}
