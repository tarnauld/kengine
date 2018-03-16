use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston::input::*;

use game::game::Game;
use game::snake::Snake;

pub fn engine(){
    let opengl = OpenGL::V3_2;

    let mut window : Window = WindowSettings::new(
            "snake",
            [500, 500]
    ).opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut app = Game{
        gl: GlGraphics::new(opengl),
        s: Snake::new()
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args(){
            app.render(&r);
        }

        if let Some(u) = e.update_args(){
            app.update(&u);
        }
    }
}
