use input::kies::Kies;
use std::collections::HashMap;

pub struct Keyboard{
    h: HashMap<Kies, &Fn()>
}

impl Keyboard{
    pub fn binding(&mut self, k: Kies, f : &Fn()){

    }
}
