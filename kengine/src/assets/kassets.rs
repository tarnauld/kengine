use assets::ksprite::Ksprite;
use std::collections::HashMap;

pub struct Kassets{
    s: HashMap<String, Ksprite>
}

impl Kassets{
        pub fn new() -> Kassets{
            Kassets{
                s: HashMap::new()
            }
        }

        pub fn add(&mut self, id: &str, ks: Ksprite){
            self.s.insert(id.to_string(), ks);
        }

        pub fn get_kassets(&mut self) -> &mut HashMap<String, Ksprite>{
            &mut self.s
        }
}
