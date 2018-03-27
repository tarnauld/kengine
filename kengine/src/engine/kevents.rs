use input::keys::Keys;

pub enum KeventType{
    COLLISION,
    KEYPRESSED
}

pub struct Kevents{
    k: Option<Keys>,
    t: Option<KeventType>
}

impl Kevents{
    pub fn new(k: Option<Keys>, t: Option<KeventType>) -> Kevents{
        Kevents{
            k: k,
            t: t
        }
    }

    pub fn get_keys(&self) -> &Option<Keys>{
        &self.k
    }

    pub fn get_keventtype(&self) -> &Option<KeventType>{
        &self.t
    }
}
