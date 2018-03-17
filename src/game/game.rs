use opengl_graphics::GlGraphics;
use piston::input::*;
use game::snake::*;
use graphics::*;

pub struct Game {
    pub gl: GlGraphics,
    pub s: Snake,
    pub w_width : u64,
    pub w_height : u64,
    pub step: u32,
    pub i : u64
}

impl Game{
    pub fn verify_collision(&mut self){
        let step = self.step as i64;
        if self.s.body[0].x * step < 0 {self.s.body[0].x = self.w_width as i64 / 10;}
        if self.s.body[0].x * step > self.w_width as i64{self.s.body[0].x = 0;}
        if self.s.body[0].y * step < 0 {self.s.body[0].y = self.w_height as i64 / 10;}
        if self.s.body[0].y * step > self.w_height as i64{self.s.body[0].y = 0;}
    }

    pub fn render(&mut self, args: &RenderArgs){
        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const SNAKE_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let nb = self.s.body.len();

        let tmp = self.s.body.to_vec();
        let step = self.step as i64;

        self.gl.draw(args.viewport(), |c, gl| {
                clear(BACKGROUND, gl);
                for i in 0..nb{
                    let square = rectangle::square(0.0, 0.0, 10.0);
                    let (x, y) = (tmp[i].x * step, tmp[i].y * step);
                    let transform = c.transform.trans(x as f64, y as f64);
                    rectangle(SNAKE_COLOR, square, transform, gl);
                }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs){
        self.i += 1;
        if self.i % 10 == 0 {self.s.add_part();}
        self.s.move_parts();
        self.verify_collision();
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
