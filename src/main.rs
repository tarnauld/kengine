extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate piston_window;

mod games;
use games::engine::engine;

fn main(){
    engine();
}
