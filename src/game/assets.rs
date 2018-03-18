use opengl_graphics::{Texture, TextureSettings};
use std::path::Path;

pub struct Assets {
    pub snake_texture : Texture,
    pub fruit_texture : Texture,
    pub ennemy_texture : Texture,
    pub texture: Texture
}

impl Assets{
    pub fn new() -> Assets{
        Assets{
            snake_texture : Texture::from_path(&Path::new("./assets/snake.png"), &TextureSettings::new()).unwrap(),
            fruit_texture : Texture::from_path(&Path::new("./assets/apple.png"), &TextureSettings::new()).unwrap(),
            ennemy_texture : Texture::from_path(&Path::new("./assets/fog.png"), &TextureSettings::new()).unwrap(),
            texture : Texture::from_path(&Path::new("./assets/ground.png"), &TextureSettings::new()).unwrap()
        }
    }
}
