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
}
