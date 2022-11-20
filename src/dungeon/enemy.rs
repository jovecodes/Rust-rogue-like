use super::{unit::Unit, position, dungeon, player, pathfinding};

pub struct Enemy {
    pub unit : Unit,
}

impl Enemy {
    pub fn new(position : position::Position) -> Enemy {
        Enemy { unit: Unit { position, sprite: 'E' } }
    }
       

    pub fn do_action(&mut self, dungeon: &dungeon::Dungeon, player: &player::Player) {
        if self.unit.position == player.get_position() {
            return; 
        }

        let direction = pathfinding::pathfind(&self.unit.position, &player.unit.position);

        self.unit.walk(position::Position::new(direction.x, direction.y), dungeon);
    }
}
