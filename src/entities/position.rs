
use serde_derive::{Serialize, Deserialize};

pub const LEFT  : Position = Position {x:-1, y: 0};
pub const RIGHT : Position = Position {x: 1, y: 0};
pub const DOWN  : Position = Position {x: 0, y: 1};
pub const UP    : Position = Position {x: 0, y:-1};
pub const STOP  : Position = Position {x: 0, y: 0};


#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Position {x: i32, y: i32}

impl Position {
    pub fn new(x: i32, y:i32) -> Position {
        Position { x, y }
    }


    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }


    pub fn set_with_int(&mut self, int : i32) -> Position {
        match int {
            0 => return LEFT,
            1 => return RIGHT,
            2 => return DOWN,
            3 => return UP,
            _ => return STOP
        }
    }


    pub fn add(&mut self, pos: Position) {
        self.x += pos.x;
        self.y += pos.y;
    } 

    pub fn get_add(&self, pos: &Position) -> Position {
        Position::new(
            self.x + pos.x,
            self.y + pos.y
        )
    }

    pub fn get_minus(&self, pos: &Position) -> Position {
        Position::new(
            self.x - pos.x,
            self.y - pos.y
        )
    }


    pub fn become_direction(&mut self) {
        if self.x.abs() > self.y.abs() {
            self.y = 0;
            self.x /= self.x.abs();
        } else {
            self.x = 0;
            self.y /= self.y.abs();
        }
    }


    pub fn distance_to(&self, pos: &Position) -> f32 {
        let x = (self.x - pos.x).pow(2);
        let y = (self.y - pos.y).pow(2);
        let total = (x + y) as f32;
        total.sqrt()
    }
}


