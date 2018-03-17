use opengl_graphics::GlGraphics;
use piston::input::*;
use game::snake::*;

pub struct Game {
    pub gl: GlGraphics,
    pub s: Snake,
    pub x: f64,
    pub y: f64
}

impl Game{
    pub fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const SNAKE_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 10.0);
        let (x, y) = (self.x * 10.0, self.y * 10.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            let transform = c.transform.trans(x, y);
            rectangle(SNAKE_COLOR, square, transform, gl);
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs){
        match self.s.direction {
            Direction::RIGHT => self.x += 1.0,
            Direction::LEFT => self.x -= 1.0,
            Direction::DOWN => self.y += 1.0,
            Direction::UP => self.y -= 1.0
        }
    }

    pub fn input(&mut self, inp: &ButtonArgs){
        let b = inp.button;
        if inp.state == ButtonState::Press{
            match b{
                 Button::Keyboard(Key::Down) => self.s.change_direction(Direction::DOWN),
                 Button::Keyboard(Key::Up) => self.s.change_direction(Direction::UP),
                 Button::Keyboard(Key::Right) => self.s.change_direction(Direction::RIGHT),
                 Button::Keyboard(Key::Left) => self.s.change_direction(Direction::LEFT),
                _ => {}
            }
        }
    }
}
