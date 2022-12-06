use std::io::{self,Read};
use crate::dungeon::dungeon;
use crate::entities::position;
use crate::lighting::light::Light;

const PLAYER_LIGHT_BRIGHTNESS: i32 = 3;


const MOVE_UP : char = 'w';
const MOVE_LEFT : char = 'a';
const MOVE_DOWN : char = 's';
const MOVE_RIGHT : char = 'd';
const MINE : char = 'm';
const PLACE_LIGHT : char = 'l';
const BUILD : char = 'b';
const QUIT : char = 'q';

enum Action {
    Walk,
    Build,
    Mine,
    PlaceLight,
}

pub struct Player {
    action: Action,
    materials: i32,
    position: position::Position,
    sprite: char,
    pub light: Light
}


impl Player {
    pub fn new(position: position::Position, materials: i32) -> Player {
        Player { 
            position, 
            sprite: '@', 
            action: Action::Walk, 
            materials,
            light: Light::new(PLAYER_LIGHT_BRIGHTNESS, position),
        }
    }
   

    pub fn do_action(&mut self, mut dungeon: &mut dungeon::Dungeon) {
        println!("Materials: {}", self.materials);
        loop { 
            let action = Player::get_input();
            let mut valid_action = true;
            match action {
                MOVE_UP => self.get_action(position::LEFT, dungeon),
                MOVE_LEFT => self.get_action(position::UP, dungeon),
                MOVE_DOWN => self.get_action(position::RIGHT, dungeon),
                MOVE_RIGHT => self.get_action(position::DOWN, dungeon),
                MINE => self.action = Action::Mine,
                BUILD => self.action = Action::Build,
                PLACE_LIGHT => self.action = Action::PlaceLight,
                QUIT => {
                    dungeon.valid = false;
                    return;
                },
                _ => valid_action = false,
            }
            if valid_action {
                break;
            }
        }
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
        dungeon: &mut dungeon::Dungeon
    ) {
        match self.action {
            Action::Walk => self.walk(direction, &dungeon),
            Action::Mine => self.mine(direction, dungeon),
            Action::Build => self.build(direction, dungeon),
            Action::PlaceLight => self.place_light(direction, dungeon),
        }
        self.action = Action::Walk;
        self.light.set_pos(self.position);
    }


    fn walk(
        &mut self, direction: position::Position, 
        dungeon: &dungeon::Dungeon
    ) {
       let future_position = self.position.get_add(&direction);
       if dungeon.does_position_have_collision(&future_position) == false {
            self.position.add(direction);
       }
    }


    fn mine(
        &mut self, 
        direction: position::Position, 
        dungeon: &mut dungeon::Dungeon
    ) {
        self.position.add(direction);

        if dungeon.does_position_have_collision(&self.position) {
            self.materials += 1;
        }

        dungeon.set_room_as_empty(self.position);
    }


    fn build(
        &mut self, 
        direction: position::Position, 
        dungeon: &mut dungeon::Dungeon
    ) {
        if self.materials <= 0 {
            return;
        }

        let build_pos = &self.position.get_add(&direction);
        
        if dungeon.does_position_have_collision(&build_pos) == true {
            return;
        }

        self.materials -= 1;
        dungeon.erase_room(build_pos);
    }

     
    fn place_light(
        &mut self, 
        direction: position::Position,
        dungeon: &mut dungeon::Dungeon
    ) {
        dungeon.lights.push(Light::new(10, self.position.get_add(&direction)));
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

    pub fn get_light(&self) -> Light {
        Light::new(self.light.get_brightness(), self.position)
    }

}
