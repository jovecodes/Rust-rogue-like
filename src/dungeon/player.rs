use std::io::{self,Read};
use crate::dungeon::dungeon;
use crate::dungeon::position;
use crate::dungeon::unit;

const MOVE_UP : char = 'w';
const MOVE_LEFT : char = 'a';
const MOVE_DOWN : char = 's';
const MOVE_RIGHT : char = 'd';
const QUIT : char = 'q';

pub struct Player {
    pub unit : unit::Unit
}


impl Player {
    pub fn new(position: position::Position) -> Player {
        Player { unit: unit::Unit::new(position, '@')}
    }
   

    pub fn do_action(&mut self, dungeon: &dungeon::Dungeon) -> bool {
        loop {   
            let action = Player::get_input();
            let mut valid_action = true;
            match action {
                MOVE_UP => self.unit.walk(position::LEFT, dungeon),
                MOVE_LEFT => self.unit.walk(position::UP, dungeon),
                MOVE_DOWN => self.unit.walk(position::RIGHT, dungeon),
                MOVE_RIGHT => self.unit.walk(position::DOWN, dungeon),
                QUIT => return false,
                _ => valid_action = false,
            }
            if valid_action {
                break;
            }
        }
        return true;
    }


    fn get_input() -> char {
        for byte in io::stdin().bytes() {
            let action = byte.unwrap() as char;
            return action;
        }
        return 'h'
    }

    pub fn get_position(&self) -> position::Position {
        self.unit.position
    }
}
