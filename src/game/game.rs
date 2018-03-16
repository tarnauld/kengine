use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::*;
use game::snake::*;

pub struct Game {
    pub gl: GlGraphics,
    pub s: Snake
}

impl Game{
    pub fn render(&mut self, args: &RenderArgs){
        self.s.change_direction(Direction::RIGHT);

        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const SNAKE_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            let transform = c.transform.trans(x, y).
                                        rot_rad(0.0).
                                        trans(-25.0, -25.0);
            rectangle(SNAKE_COLOR, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs){
        match self.s.direction {
            Direction::RIGHT => println!("0"),
            Direction::LEFT => println!("0"),
            Direction::DOWN => println!("0"),
            Direction::UP => println!("0")
        }
    }
}
