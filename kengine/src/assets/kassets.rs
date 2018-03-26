use assets::ksprite::Ksprite;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Kassets{
    s: Vec<Rc<RefCell<Ksprite>>>
}

impl Kassets{
        pub fn new() -> Kassets{
            Kassets{
                s: Vec::new()
            }
        }

        pub fn add(&mut self, ks: Rc<RefCell<Ksprite>>){
            self.s.push(ks);
        }

        pub fn get_kassets(&mut self) -> &mut Vec<Rc<RefCell<Ksprite>>>{
            &mut self.s
        }
}
