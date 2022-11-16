use std::io::{self,Read};
use crate::dungeon::dungeon;
use crate::dungeon::position;

const MOVE_UP : char = 'w';
const MOVE_LEFT : char = 'a';
const MOVE_DOWN : char = 's';
const MOVE_RIGHT : char = 'd';
const QUIT : char = 'q';

pub struct Player {
    position: position::Position,
    sprite: char,
}


impl Player {
    pub fn new(position: position::Position) -> Player {
        Player { position: position, sprite: '@'}
    }
   

    pub fn do_action(&mut self, dungeon: &dungeon::Dungeon) -> bool {
        loop {   
            let action = Player::get_input();
            let mut valid_action = true;
            match action {
                MOVE_UP => self.walk(position::LEFT, dungeon),
                MOVE_LEFT => self.walk(position::UP, dungeon),
                MOVE_DOWN => self.walk(position::RIGHT, dungeon),
                MOVE_RIGHT => self.walk(position::DOWN, dungeon),
                QUIT => return false,
                _ => valid_action = false,
            }
            if valid_action {
                break;
            }
        }
        return true;
    }


    pub fn look(&self, dungeon: &dungeon::Dungeon) {
        dungeon.print_with_player(&self.position, &self.sprite);
    }


    fn get_input() -> char {
        for byte in io::stdin().bytes() {
            let action = byte.unwrap() as char;
            return action;
        }
        return 'h'
    }


    fn walk(&mut self, direction: position::Position, dungeon: &dungeon::Dungeon) {
       let future_position = position::Position::new(
           self.position.x + direction.x,
           self.position.y + direction.y
        );
       if dungeon.does_position_have_collision(&future_position) == false {
            self.position.add(direction);
       }
    }
}
