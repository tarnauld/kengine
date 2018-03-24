use opengl_graphics::{Texture, TextureSettings};
use std::path::Path;

pub struct Ktexture{
    pub t: Texture
}

impl Ktexture{
    pub fn new(p: &str) -> Ktexture{
        Ktexture{
            t: Texture::from_path(&Path::new(&p), &TextureSettings::new()).unwrap()
        }
    }

    pub fn get_texture(&self) -> &Texture{
        &self.t
    }
}
