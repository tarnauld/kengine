use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;

use game::game::Game;

pub fn engine(){
    let opengl = OpenGL::V3_2;

    let s = 30;
    let w = 25 * s;
    let h = 25 * s;


    let mut window : Window = WindowSettings::new("snake",[w, h])
                                                .opengl(opengl)
                                                .exit_on_esc(true)
                                                .build()
                                                .unwrap();
    let mut app = Game::new(opengl, w, h, s);

    let mut events = Events::new(EventSettings::new());
    events.set_ups(5);

    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args(){
            app.render(&r);
        }

        if let Some(u) = e.update_args(){
            app.update(&u);
        }

        if let Some(i) = e.button_args(){
            app.input(&i);
        }
    }
}
