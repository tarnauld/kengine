use generics::kcoord::Kcoord;
use generics::kdirection::Kdirection;
use assets::ktexture::Ktexture;

pub struct Ksprite{
    t: Option<Ktexture>,
    c: Kcoord,
    s: f64,
    d: Option<Kdirection>
}

impl Ksprite{
    pub fn new(x: f64, y: f64, s: f64) -> Ksprite{
        Ksprite{
            t: None,
            c: Kcoord::new(x, y),
            s: s,
            d: None
        }
    }

    pub fn add_texture(&mut self, t: Ktexture){
        self.t = Some(t);
    }

    pub fn add_direction(&mut self, d: Kdirection){
        self.d = Some(d);
    }

    pub fn get_kdirection(&self) -> Kdirection{
        return self.d.clone().unwrap();
    }

    pub fn set_kcoord_x(&mut self, x : f64){
        self.c.x = x;
    }

    pub fn set_kcoord_y(&mut self, y : f64){
        self.c.y = y;
    }

    pub fn set_kcoord(&mut self, x: f64, y: f64){
        self.c.x = x;
        self.c.y = y;
    }

    pub fn get_ksprite(&self) -> (&Option<Ktexture>, f64, f64){
        return (&self.t, self.c.x, self.c.y);
    }

    pub fn get_kcoord(&self) -> &Kcoord{
        &self.c
    }

    pub fn move_ksprite(&mut self){
        match self.d {
            None => return,
            Some(Kdirection::RIGHT) => self.c.x += 1.0 * self.s,
            Some(Kdirection::LEFT) => self.c.x -= 1.0 * self.s,
            Some(Kdirection::DOWN) => self.c.y += 1.0 * self.s,
            Some(Kdirection::UP) => self.c.y -= 1.0 * self.s
        }
    }
}
