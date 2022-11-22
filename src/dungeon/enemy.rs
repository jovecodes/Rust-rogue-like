use super::{position, dungeon, player, pathfinding};


pub struct Enemy {
    pub position: position::Position,
    pub sprite: char,
}

impl Enemy {
    pub fn new(position: position::Position) -> Enemy {
        Enemy { position, sprite: 'E' }
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
       

    pub fn do_action(&mut self, dungeon: &dungeon::Dungeon, player: &player::Player) {
        if &self.position == player.get_position() {
            return; 
        }

        let direction = pathfinding::pathfind(&self.position, player.get_position());

        self.walk(position::Position::new(direction.x, direction.y), dungeon);
    }


    pub fn get_position(&self) -> &position::Position {
        &self.position
    }
}
