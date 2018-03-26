extern crate kengine;

use kengine::engine::kengine::Kengine;
use kengine::assets::ksprite::Ksprite;
use kengine::assets::ktexture::Ktexture;
use kengine::generics::kdirection::Kdirection;
use kengine::input::keys::Keys;

fn main(){
    let mut kengine : Kengine = Kengine::new("Snake", 1200, 750, 10);

    let mut ground = Ksprite::new(0., 0., 0.);
    let ground_img = Ktexture::new("./gamesnake/assets/ground.png");
    ground.add_texture(ground_img);
    kengine.add_ksprite("ground", ground);

    let mut snake = Ksprite::new(500., 300., 30.);
    let snake_img = Ktexture::new("./gamesnake/assets/snake.png");
    snake.add_texture(snake_img);
    kengine.add_ksprite("snake", snake);

    loop{
        let events = kengine.run();
        match *events.get_keys(){
            Some(Keys::Left) => {
                kengine.get_ksprite("snake").add_direction(Kdirection::LEFT);
            },
            Some(Keys::Right) => {
                kengine.get_ksprite("snake").add_direction(Kdirection::RIGHT);
            },
            Some(Keys::Up) => {
                kengine.get_ksprite("snake").add_direction(Kdirection::UP);
            },
            Some(Keys::Down) => {
                kengine.get_ksprite("snake").add_direction(Kdirection::DOWN);
            },
            Some(Keys::Escape) => {break;},
            _ => {}
        }
    }
}
