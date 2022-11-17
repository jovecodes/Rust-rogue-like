use super::{unit::Unit, position, dungeon};

pub struct Enemy {
    pub unit : Unit,
}

impl Enemy {
    pub fn new(position : position::Position) -> Enemy {
        Enemy { unit: Unit { position, sprite: 'E' } }
    }
   

    pub fn do_action(&mut self, dungeon: &dungeon::Dungeon) {

    }
}
