#[derive(Debug, Clone, Copy)]
pub struct Kcoord{
    pub x: f64,
    pub y: f64
}

impl Kcoord{
    pub fn new(x: f64, y: f64) -> Kcoord{
        Kcoord{
            x: x,
            y: y
        }
    }
}
