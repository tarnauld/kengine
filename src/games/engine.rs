use piston_window::*;
use opengl_graphics::OpenGL;

use games::game::Game;

pub fn engine(){
    let opengl = OpenGL::V3_2;

    let tile_size = 30;
    let window_width = 1200;
    let window_height = 750;


    let mut window : PistonWindow = WindowSettings::new("snake",[window_width, window_height])
                                                    .opengl(opengl)
                                                    .exit_on_esc(true)
                                                    .build()
                                                    .unwrap();

    let mut app = Game::new(opengl, window_width, window_height, tile_size);

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
