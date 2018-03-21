#[derive(Clone)]
pub struct Kcoord{
    pub x: i64,
    pub y: i64
}

impl Kcoord{
    pub fn new(x: i64, y: i64) -> Kcoord{
        Kcoord{
            x: x,
            y: y
        }
    }
}
