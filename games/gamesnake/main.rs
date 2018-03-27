extern crate kengine;

use kengine::engine::kengine::Kengine;
use kengine::assets::ksprite::Ksprite;
use kengine::assets::ktexture::Ktexture;
use kengine::generics::kdirection::Kdirection;
use kengine::input::keys::Keys;
use kengine::generics::kcollide::collide_with_window;
use kengine::generics::kcollide::WindowBorders;
use kengine::engine::kevents::KeventType;

fn main(){
    let mut kengine : Kengine = Kengine::new("Snake", 1200, 750, 10);

    let mut ground = Ksprite::new(0., 0., 0.);
    let ground_img = Ktexture::new("./gamesnake/assets/ground.png");
    ground.add_texture(ground_img);
    kengine.add_ksprite("ground", ground);

    let mut snake = Ksprite::new(0., 0., 30.);
    let snake_img = Ktexture::new("./gamesnake/assets/snake.png");
    snake.add_texture(snake_img);
    snake.add_direction(Kdirection::RIGHT);
    kengine.add_ksprite("snake", snake);

    loop{
        let events = kengine.run();

        match *events.get_keventtype(){
            Some(KeventType::COLLISION) => {
                match collide_with_window(kengine.get_ksprite("snake").get_kcoord(), 1200 as f64, 750 as f64){
                    WindowBorders::LEFT => kengine.get_ksprite("snake").set_kcoord_x(1200.),
                    WindowBorders::RIGHT => kengine.get_ksprite("snake").set_kcoord_x(0.),
                    WindowBorders::UP => kengine.get_ksprite("snake").set_kcoord_y(750.),
                    WindowBorders::DOWN => kengine.get_ksprite("snake").set_kcoord_y(0.),
                    WindowBorders::CENTER => {}
                }
            },
            Some(KeventType::KEYPRESSED) => {
                match *events.get_keys(){
                    Some(Keys::Left) => {
                        if kengine.get_ksprite("snake").get_kdirection() != Kdirection::RIGHT{
                            kengine.get_ksprite("snake").add_direction(Kdirection::LEFT);
                        }
                    },
                    Some(Keys::Right) => {
                        if kengine.get_ksprite("snake").get_kdirection() != Kdirection::LEFT{
                            kengine.get_ksprite("snake").add_direction(Kdirection::RIGHT);
                        }
                    },
                    Some(Keys::Up) => {
                        if kengine.get_ksprite("snake").get_kdirection() != Kdirection::DOWN{
                            kengine.get_ksprite("snake").add_direction(Kdirection::UP);
                        }
                    },
                    Some(Keys::Down) => {
                        if kengine.get_ksprite("snake").get_kdirection() != Kdirection::UP{
                            kengine.get_ksprite("snake").add_direction(Kdirection::DOWN);
                        }
                    },
                    Some(Keys::Escape) => {break;},
                    _ => {}
                }
            },
            _ => {}
        }
    }
}
