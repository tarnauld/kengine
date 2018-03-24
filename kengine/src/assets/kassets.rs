use assets::ksprite::Ksprite;

pub struct Kassets{
    s: Vec<Ksprite>
}

impl Kassets{
        pub fn new() -> Kassets{
            Kassets{
                s: Vec::new()
            }
        }

        pub fn add(&mut self, ks: Ksprite){
            self.s.push(ks);
        }

        pub fn get_kassets(&mut self) -> &mut Vec<Ksprite>{
            &mut self.s
        }
}
