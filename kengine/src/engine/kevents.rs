use input::keys::Keys;

pub struct Kevents{
    k: Option<Keys>
}

impl Kevents{
    pub fn new(k: Option<Keys>) -> Kevents{
        Kevents{
            k: k
        }
    }

    pub fn get_keys(&self) -> &Option<Keys>{
        &self.k
    }
}
