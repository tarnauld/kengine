use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use game::snake::*;
use graphics::*;
use game::coord::Coord;
use rand;
use rand::Rng;
use game::assets::Assets;

pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub fruits: Vec<Coord>,
    pub ennemies: Vec<Coord>,
    pub w_width : u32,
    pub w_height : u32,
    pub assets: Assets,
    pub score: u32,
    pub step: u32,
    pub ups: u64,
    pub i : u64
}

impl Game{
    pub fn new(opengl: OpenGL, w: u32, h: u32, s: u32) -> Game{
        Game{
            gl: GlGraphics::new(opengl),
            snake: Snake::new(),
            fruits: vec![],
            ennemies: vec![],
            w_width: w,
            w_height: h,
            assets : Assets::new(),
            score: 0,
            step: s,
            ups: 5,
            i: 0
        }
    }

    pub fn verify_collision(&mut self){
        let step = self.step as i64;
        let mut v : Vec<usize> = Vec::new();
        let mut to_remove = 0;

        if self.snake.body[0].x * step < 0 {self.snake.body[0].x = self.w_width as i64 / self.step as i64;}
        if self.snake.body[0].x * step > self.w_width as i64{self.snake.body[0].x = 0;}
        if self.snake.body[0].y * step < 0 {self.snake.body[0].y = self.w_height as i64 / self.step as i64;}
        if self.snake.body[0].y * step > self.w_height as i64{self.snake.body[0].y = 0;}

        if self.snake.body.len() > 1{
            for i in 1..self.snake.body.len(){
                if self.snake.body[0].x == self.snake.body[i].x && self.snake.body[0].y == self.snake.body[i].y{
                    to_remove = i;
                }
            }
        }

        if to_remove != 0{
            for i in (to_remove..self.snake.body.len()).rev(){
                self.snake.body.remove(i);
                self.score -= 1;
            }
        }

        for i in 0..self.fruits.len(){
            if self.snake.body[0].x == self.fruits[i].x && self.snake.body[0].y == self.fruits[i].y{
                v.push(i);
                self.score += 1;
                self.snake.add_part();
            }
        }

        for i in 0..v.len(){
            self.fruits.remove(v[i]);
        }

        for i in 0..self.ennemies.len(){
            if self.snake.body[0].x == self.ennemies[i].x && self.snake.body[0].y == self.ennemies[i].y{
                self.game_over();
                return;
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs){
        let nb = self.snake.body.len();

        let snake = self.snake.body.to_vec();
        let step = self.step as i64;
        let fruits = self.fruits.to_vec();
        let ennemies = self.ennemies.to_vec();
        let size = self.step as f64;
        let texture = &self.assets.texture;
        let snake_color = self.assets.snake_color;
        let fruit_color = self.assets.fruit_color;
        let ennemy_color = self.assets.ennemy_color;

        self.gl.draw(args.viewport(), |c, gl| {
                image(texture, c.transform, gl);
                for i in 0..nb{
                    let square = rectangle::square(0.0, 0.0, size);
                    let (x, y) = (snake[i].x * step, snake[i].y * step);
                    let transform = c.transform.trans(x as f64, y as f64);
                    rectangle(snake_color, square, transform, gl);
                }
                for i in 0..fruits.len(){
                    let square = rectangle::square(0.0, 0.0, size);
                    let (x, y) = (fruits[i].x * step, fruits[i].y * step);
                    let transform = c.transform.trans(x as f64, y as f64);
                    rectangle(fruit_color, square, transform, gl);
                }
                for i in 0..ennemies.len(){
                    let square = rectangle::square(0.0, 0.0, size);
                    let (x, y) = (ennemies[i].x * step, ennemies[i].y * step);
                    let transform = c.transform.trans(x as f64, y as f64);
                    rectangle(ennemy_color, square, transform, gl);
                }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs){
        self.i += 1;
        if self.i % 40 == 0{
            if self.fruits.len() > 0{
                self.fruits.remove(0);
            }
        }
        if self.i % 60 == 0{
            if self.ennemies.len() > 0{
                self.ennemies.remove(0);
            }
        }
        if self.i % 20 == 0 {
            let (x, y) = self.choose_random();
            self.fruits.push(Coord{
                x: x,
                y: y
            });
        }
        if self.i % 30 == 0 {
            let (x, y) = self.choose_random();
            self.ennemies.push(Coord{
                x: x,
                y: y
            });
        }
        self.snake.move_parts();
        self.verify_collision();
    }

    pub fn input(&mut self, inp: &ButtonArgs){
        let b = inp.button;
        if inp.state == ButtonState::Press{
            match b{
                 Button::Keyboard(Key::Down) => self.snake.change_direction(Direction::DOWN),
                 Button::Keyboard(Key::Up) => self.snake.change_direction(Direction::UP),
                 Button::Keyboard(Key::Right) => self.snake.change_direction(Direction::RIGHT),
                 Button::Keyboard(Key::Left) => self.snake.change_direction(Direction::LEFT),
                _ => {}
            }
        }
    }

    fn choose_random(&mut self) -> (i64, i64){
        (rand::thread_rng().gen_range(0, self.w_width as i64 / self.step as i64),
        rand::thread_rng().gen_range(0, self.w_height as i64 / self.step as i64))
    }

    pub fn game_over(&mut self){
        self.fruits.clear();
        self.ennemies.clear();
        self.snake.body.clear();
        self.snake = Snake::new();
        self.i = 0;
    }
}
