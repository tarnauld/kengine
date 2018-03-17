use game::coord::Coord;

pub struct Snake{
    pub body: Vec<Coord>,
    pub direction: Direction
}

 #[derive(PartialEq)]
pub enum Direction {
    RIGHT,
    LEFT,
    DOWN,
    UP
}

impl Snake{
    pub fn change_direction(&mut self, direction: Direction){
        match direction {
            Direction::LEFT => {
                if self.direction == Direction::RIGHT {
                    return;
                }
            }
            Direction::RIGHT => {
                if self.direction == Direction::LEFT {
                    return;
                }
            }
            Direction::DOWN => {
                if self.direction == Direction::UP {
                    return;
                }
            }
            Direction::UP => {
                if self.direction == Direction::DOWN {
                    return;
                }
            }
        }
        self.direction = direction;
    }

    pub fn add_part(&mut self){
        let x = self.body[self.body.len() - 1 as usize].x;
        let y = self.body[self.body.len() - 1 as usize].y;

        self.body.push(Coord{
            x: x,
            y: y
        });
    }

    pub fn move_parts(&mut self){
        for i in (1..self.body.len()).rev() {
            self.body[i as usize].x = self.body[(i - 1) as usize].x;
            self.body[i as usize].y = self.body[(i - 1) as usize].y;
        }
        match self.direction {
            Direction::RIGHT => self.body[0].x += 1,
            Direction::LEFT => self.body[0].x -= 1,
            Direction::DOWN => self.body[0].y += 1,
            Direction::UP => self.body[0].y -= 1
        }
    }

    pub fn new() -> Snake{
        Snake{
            body: vec![Coord{
                x: 0, y: 0
            }],
            direction: Direction::RIGHT
        }
    }
}
