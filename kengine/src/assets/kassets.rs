use assets::ksprite::Ksprite;
use linked_hash_map::LinkedHashMap;

pub struct Kassets{
    s: LinkedHashMap<String, Ksprite>
}

impl Kassets{
        pub fn new() -> Kassets{
            Kassets{
                s: LinkedHashMap::new()
            }
        }

        pub fn add(&mut self, id: String, ks: Ksprite){
            self.s.insert(id.to_string(), ks);
        }

        pub fn get_kassets(&mut self) -> &mut LinkedHashMap<String, Ksprite>{
            &mut self.s
        }
}
