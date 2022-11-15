pub const LEFT  : Position = Position {x:-1, y: 0};
pub const RIGHT : Position = Position {x: 1, y: 0};
pub const DOWN  : Position = Position {x: 0, y: 1};
pub const UP    : Position = Position {x: 0, y:-1};
pub const STOP  : Position = Position {x: 0, y: 0};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Position {pub x: i32,pub y: i32}

impl Position {
    pub fn new(x: i32, y:i32) -> Position {
        Position { x: x, y: y }
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
}

