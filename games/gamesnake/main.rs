extern crate kengine;

use kengine::engine::kengine::*;

fn main(){
    let mut kengine : Kengine = Kengine::new("Snake".to_string(), 1200, 750);
    kengine.run();
}
