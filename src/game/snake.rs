use std::collections::LinkedList;

pub struct Snake{
    pub body: LinkedList<Snake>,
    pub direction: Direction
}

#[derive(Debug)]
pub enum Direction {
    RIGHT,
    LEFT,
    DOWN,
    UP
}

impl Snake{
    pub fn change_direction(&mut self, direction: Direction){
        self.direction = direction;
    }

    pub fn new() -> Snake{
        Snake{
            body: LinkedList::new(),
            direction: Direction::RIGHT
        }
    }
}
