use std::collections::LinkedList;

pub struct Cord{
    x: i32,
    y: i32
}

pub struct Snake{
    pub body: LinkedList<Cord>,
    pub direction: Direction
}

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
