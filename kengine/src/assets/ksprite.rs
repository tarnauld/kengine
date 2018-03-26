use generics::kcoord::Kcoord;
use generics::kdirection::Kdirection;
use assets::ktexture::Ktexture;

pub struct Ksprite{
    t: Option<Ktexture>,
    c: Kcoord,
    d: Option<Kdirection>
}

impl Ksprite{
    pub fn new(x: f64, y: f64) -> Ksprite{
        Ksprite{
            t: None,
            c: Kcoord::new(x, y),
            d: None
        }
    }

    pub fn add_texture(&mut self, t: Ktexture){
        self.t = Some(t);
    }

    pub fn add_direction(&mut self, d: Kdirection){
        self.d = Some(d);
    }

    pub fn get_kdirection(&self) -> &Option<Kdirection>{
        return &self.d;
    }

    pub fn get_ksprite(&self) -> (&Option<Ktexture>, f64, f64){
        return (&self.t, self.c.x, self.c.y);
    }

    pub fn get_kcoord(self) -> Kcoord{
        self.c
    }

    pub fn move_ksprite(&mut self){
        match self.d {
            None => return,
            Some(Kdirection::RIGHT) => self.c.x += 1.0,
            Some(Kdirection::LEFT) => self.c.x -= 1.0,
            Some(Kdirection::DOWN) => self.c.y += 1.0,
            Some(Kdirection::UP) => self.c.y -= 1.0
        }
    }
}
