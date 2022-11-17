use crate::dungeon::position;
use crate::dungeon::dungeon;


pub struct Unit { 
    pub position: position::Position,
    pub sprite: char,
}


impl Unit {
    pub fn new(position: position::Position, sprite: char) -> Unit {
        Unit { position: position, sprite: sprite}
    }

    pub fn walk(
        &mut self, direction: position::Position, 
        dungeon: &dungeon::Dungeon
    ) {
       let future_position = position::Position::new(
           self.position.x + direction.x,
           self.position.y + direction.y
        );
       if dungeon.does_position_have_collision(&future_position) == false {
            self.position.add(direction);
       }
    }
}
