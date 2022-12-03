use std::io::{self,Read};
use crate::dungeon::dungeon;
use crate::dungeon::position;
use crate::dungeon::room;


const MOVE_UP : char = 'w';
const MOVE_LEFT : char = 'a';
const MOVE_DOWN : char = 's';
const MOVE_RIGHT : char = 'd';
const MINE : char = 'm';
const BUILD : char = 'b';
const QUIT : char = 'q';

enum Action {
    Walk,
    Build,
    Mine,
}

pub struct Player {
    action: Action,
    materials: i32,
    position: position::Position,
    sprite: char,
}


impl Player {
    pub fn new(position: position::Position, materials: i32) -> Player {
        Player { position, sprite: '@', action: Action::Walk, materials}
    }
   

    pub fn do_action(&mut self, mut dungeon: dungeon::Dungeon) -> dungeon::Dungeon {
        println!("Materials: {}", self.materials);
        loop { 
            let action = Player::get_input();
            let mut valid_action = true;
            match action {
                MOVE_UP => dungeon = self.get_action(position::LEFT, dungeon),
                MOVE_LEFT => dungeon = self.get_action(position::UP, dungeon),
                MOVE_DOWN => dungeon = self.get_action(position::RIGHT, dungeon),
                MOVE_RIGHT => dungeon = self.get_action(position::DOWN, dungeon),
                MINE => self.action = Action::Mine,
                BUILD => self.action = Action::Build,
                QUIT => {
                    dungeon.valid = false;
                    return dungeon;
                },
                _ => valid_action = false,
            }
            if valid_action {
                break;
            }
        }
        return dungeon;
    }

    
    fn get_input() -> char {
        for byte in io::stdin().bytes() {
            let action = byte.unwrap() as char;
            return action;
        }
        return 'x'
    }


    fn get_action(
        &mut self, 
        direction: position::Position, 
        dungeon: dungeon::Dungeon
    ) -> dungeon::Dungeon {
        let mut new_dungeon = dungeon;
        match self.action {
            Action::Walk => self.walk(direction, &new_dungeon),
            Action::Mine => new_dungeon = self.mine(direction, new_dungeon),
            Action::Build => new_dungeon = self.build(direction, new_dungeon),
        }
        self.action = Action::Walk;
        new_dungeon
    }


    fn walk(
        &mut self, direction: position::Position, 
        dungeon: &dungeon::Dungeon
    ) {
       let future_position = self.position.plus(&direction);
       if dungeon.does_position_have_collision(&future_position) == false {
            self.position.add(direction);
       }
    }


    fn mine(
        &mut self, 
        direction: position::Position, 
        dungeon: dungeon::Dungeon
    ) -> dungeon::Dungeon {

        let mut new_dungeon = dungeon;

        self.position.x += direction.x;
        self.position.y += direction.y;

        if new_dungeon.does_position_have_collision(&self.position) {
            self.materials += 1;
        }

        new_dungeon.set_room(
            &self.position, 
            room::Room::new(
                '.', 
                self.position, 
                0
            )
        );

        new_dungeon
    }


    fn build(
        &mut self, 
        direction: position::Position, 
        dungeon: dungeon::Dungeon
    ) -> dungeon::Dungeon {

        let build_pos = &position::Position::new(
            self.get_position().x + direction.x,
            self.get_position().y + direction.y,
        );
        
        if dungeon.does_position_have_collision(&build_pos) == false {
            if self.materials <= 0 {
                return dungeon;
            }
            self.materials -= 1;
        }
        let mut new_dungeon = dungeon;
        new_dungeon.erase_room(build_pos);

        new_dungeon
    }
    

    
    pub fn get_position(&self) -> &position::Position {
        &self.position
    }

    pub fn get_materials(&self) -> i32 {
        self.materials
    }

    pub fn get_sprite(&self) -> char {
        self.sprite
    }

}
