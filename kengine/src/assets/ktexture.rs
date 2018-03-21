use opengl_graphics::{Texture, TextureSettings};
use std::path::Path;

pub struct Ktexture{
    t: Option<Texture>
}

impl Ktexture {
    pub fn new() -> Ktexture{
        Ktexture{
            t: None
        }
    }

    pub fn load_ktexture(p: String) -> Ktexture{
        Ktexture{
            t: Some(Texture::from_path(&Path::new(&p), &TextureSettings::new()).unwrap())
        }
    }
}
