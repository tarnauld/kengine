use piston::input::*;
use assets::kassets::Kassets;

pub struct Kgame {
    a: Kassets
}

impl Kgame{
    pub fn new() -> Kgame{
        Kgame{
            a: Kassets::new()
        }
    }

    pub fn render(&mut self, args: &RenderArgs){

    }

    pub fn update(&mut self, _args: &UpdateArgs){

    }

    pub fn input(&mut self, inp: &ButtonArgs){

    }
}
