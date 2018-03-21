use generics::kcoord::Kcoord;
use assets::ktexture::Ktexture;

pub struct Ksprite{
    t: Ktexture,
    c: Kcoord
}

impl Ksprite{
    pub fn new(x: i64, y: i64) -> Ksprite{
        Ksprite{
            t: Ktexture::new(),
            c: Kcoord::new(x, y)
        }
    }

    pub fn add_texture(&mut self, t: Ktexture){
        self.t = t;
    }

    pub fn get_texture(&mut self) -> &Ktexture{
        let t = &self.t;
        &t
    }
}
