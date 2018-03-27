use assets::kassets::Kassets;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use graphics::*;
use input::keys::convert_key;
use engine::kevents::Kevents;
use engine::kevents::KeventType;
use generics::kcollide::collide_with_window;
use generics::kcollide::WindowBorders;

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

    pub fn update(&mut self, _args: &UpdateArgs, w: f64, h: f64) -> Kevents{
        let mut m : Option<Kevents> = Some(Kevents::new(None, None));
        for (_i, o) in self.a.get_kassets(){
            o.move_ksprite();
            match collide_with_window(o.get_kcoord(), w, h){
                WindowBorders::CENTER => {},
                _ => m = Some(Kevents::new(None, Some(KeventType::COLLISION))),
            }
        }
        return m.unwrap();
    }

    pub fn input(&mut self, inp: &ButtonArgs) -> Kevents{
        let b = inp.button;

        if inp.state == ButtonState::Press{
            Kevents::new(convert_key(&b), Some(KeventType::KEYPRESSED))
        }else{
            Kevents::new(None, None)
        }
    }
}
