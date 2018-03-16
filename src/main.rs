extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod game;
use game::engine::engine;

fn main(){
    engine();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
