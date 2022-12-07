use crate::{dungeon::dungeon::Dungeon, entities::entity};

use super::{position::Position, entity::Entity};

#[derive(Clone)]
pub struct Robot {
    entity: Entity,
    pattern: Vec<char>,
    current_step: usize,
    sprite: char,
}

impl Robot {
   pub fn new(position: Position) -> Robot {
        Robot { entity: Entity::new(position, 0), pattern: Vec::new(), current_step: 0, sprite: 'R' }
   }

   pub fn get_position(&self) -> Position {
       self.entity.get_position()
   }

   pub fn get_sprite(&self) -> char {
       self.sprite
   }

   pub fn add_to_pattern(&mut self, pattern_piece: char) {
        if entity::is_action_valid(&pattern_piece) {
            println!("Pattern: [{}]", pattern_piece);
            self.pattern.push(pattern_piece);
        }
   }

   pub fn do_action(&mut self, dungeon: &mut Dungeon) {
       println!("{:?}", self.pattern);
       if self.pattern.is_empty() {
           return;
       }
       self.entity.do_action(self.pattern[self.current_step], dungeon);
       self.tick_step();
   }

   fn tick_step(&mut self) {
       if self.current_step >= self.pattern.len() - 1 {
           self.current_step = 0;
       } else {
           self.current_step += 1;
       }
   }
}
