use generics::kcoord::Kcoord;
use assets::ktexture::Ktexture;

pub struct Ksprite{
    t: Option<Ktexture>,
    c: Kcoord
}

impl Ksprite{
    pub fn new(x: f64, y: f64) -> Ksprite{
        Ksprite{
            t: None,
            c: Kcoord::new(x, y)
        }
    }

    pub fn add_texture(&mut self, t: Ktexture){
        self.t = Some(t);
    }

    pub fn get_ksprite(&self) -> (&Option<Ktexture>, f64, f64){
        return (&self.t, self.c.x, self.c.y);
    }

    pub fn get_kcoord(self) -> Kcoord{
        self.c
    }
}
