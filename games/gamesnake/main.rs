extern crate kengine;

use kengine::engine::kengine::*;
use kengine::assets::ksprite::Ksprite;
use kengine::assets::ktexture::Ktexture;

fn main(){
    let mut kengine : Kengine = Kengine::new("Snake", 1200, 750);

    let mut snake = Ksprite::new(0., 0.);
    let snake_img = Ktexture::new("./gamesnake/assets/ground.png");
    snake.add_texture(snake_img);
    kengine.add_ksprite(snake);

    let mut ground = Ksprite::new(0., 0.);
    let ground_img = Ktexture::new("./gamesnake/assets/snake.png");
    ground.add_texture(ground_img);
    kengine.add_ksprite(ground);

    kengine.run();
}
