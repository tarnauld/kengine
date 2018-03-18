extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate piston_window;

mod game;
use game::engine::engine;

fn main(){
    engine();
}
