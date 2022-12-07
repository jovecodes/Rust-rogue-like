use super::{position, player, pathfinding, entity_manager::EntityManager};
use crate::dungeon::dungeon;

#[derive(Clone, Copy)]
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
        dungeon: &dungeon::Dungeon,
        manager: &EntityManager,
    ) {
       let future_position = self.position.get_add(&direction);
       if manager.does_position_have_collision(&future_position) == true {
            return;
       }
       if dungeon.does_position_have_collision(&future_position) == true {
           return;
       }
        self.position.add(direction);
    }
       

    pub fn do_action(
        &mut self, 
        dungeon: &dungeon::Dungeon, 
        player: &player::Player,
        manager: &EntityManager,
    ) {
        if self.position == player.get_position() {
            return; 
        }

        let direction = pathfinding::pathfind(&self.position, &player.get_position());

        self.walk(direction, dungeon, manager);
    }


    pub fn get_position(&self) -> &position::Position {
        &self.position
    }
}
