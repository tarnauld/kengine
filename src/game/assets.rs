use opengl_graphics::{Texture, TextureSettings};
use std::path::Path;

pub struct Assets {
    pub snake_color : [f32; 4],
    pub fruit_color : [f32; 4],
    pub ennemy_color : [f32; 4],
    pub texture: Texture
}

impl Assets{
    pub fn new() -> Assets{
        Assets{
            snake_color : [1.0, 1.0, 1.0, 1.0],
            fruit_color : [1.0, 0.0, 0.0, 1.0],
            ennemy_color : [0.0, 0.0, 1.0, 1.0],
            texture : Texture::from_path(&Path::new("./assets/ground.png"), &TextureSettings::new()).unwrap()
        }
    }
}
