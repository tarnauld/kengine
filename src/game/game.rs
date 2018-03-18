use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use game::snake::*;
use graphics::*;
use game::coord::Coord;
use game::assets::Assets;
use game::collide::*;
use game::utils::choose_random;

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
            i: 0
        }
    }

    pub fn verify_collision(&mut self){
        let mut v : Vec<usize> = Vec::new();

        collide_with_window(&mut self.snake.body[0], self.w_width as i64, self.w_height as i64, self.step as i64);

        if self.snake.body.len() > 1{
            let to_remove = collide_with_head(&self.snake.body[0], &self.snake.body);
            if to_remove > 0 {
                for i in (to_remove..self.snake.body.len() as i32).rev(){
                    self.snake.body.remove(i as usize);
                    self.score -= 1;
                }
            }
        }

        for i in 0..self.fruits.len(){
            if collision_exists(&mut self.snake.body[0], &self.fruits[i]){
                v.push(i);
                self.score += 1;
                self.snake.add_part();
            }
        }

        for i in 0..v.len(){
            self.fruits.remove(v[i]);
        }

        for i in 0..self.ennemies.len(){
            if  collision_exists(&mut self.snake.body[0], &self.ennemies[i]){
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
        let texture = &self.assets.texture;
        let snake_texture = &self.assets.snake_texture;
        let fruit_texture = &self.assets.fruit_texture;
        let ennemy_texture = &self.assets.ennemy_texture;

        self.gl.draw(args.viewport(), |c, gl| {
                image(texture, c.transform, gl);
                for i in 0..nb{
                    let (x, y) = (snake[i].x * step, snake[i].y * step);
                    image(snake_texture, c.transform.trans(x as f64, y as f64), gl);
                }
                for i in 0..fruits.len(){
                    let (x, y) = (fruits[i].x * step, fruits[i].y * step);
                    image(fruit_texture, c.transform.trans(x as f64, y as f64), gl);
                }
                for i in 0..ennemies.len(){
                    let (x, y) = (ennemies[i].x * step, ennemies[i].y * step);
                    image(ennemy_texture, c.transform.trans(x as f64, y as f64), gl);
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
            let (x, y) = choose_random(self.w_width as i64, self.w_height as i64, self.step as i64);
            self.fruits.push(Coord{
                x: x,
                y: y
            });
        }
        if self.i % 30 == 0 {
            let (x, y) = choose_random(self.w_width as i64, self.w_height as i64, self.step as i64);
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

    pub fn game_over(&mut self){
        self.fruits.clear();
        self.ennemies.clear();
        self.snake.body.clear();
        self.snake = Snake::new();
        self.i = 0;
    }
}
