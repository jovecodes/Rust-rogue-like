use std::io::{self,Read};
use crate::dungeon::dungeon;
use crate::entities::position;
use crate::lighting::light::Light;

use super::entity::Entity;

const PLAYER_LIGHT_BRIGHTNESS: i32 = 3;


pub struct Player {
    entity: Entity,
    sprite: char,
    pub light: Light
}


impl Player {
    pub fn new(position: position::Position, materials: i32) -> Player {
        Player { 
            entity: Entity::new(position, materials),
            sprite: '@', 
            light: Light::new(PLAYER_LIGHT_BRIGHTNESS, position),
        }
    }
   

    pub fn do_action(&mut self, dungeon: &mut dungeon::Dungeon) {
        println!("Materials: {}", self.entity.get_materials());
        loop { 
            let action = Player::get_input();
            let valid = self.entity.do_action(action, dungeon);
            if valid {
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


    pub fn get_position(&self) -> position::Position {
        self.entity.get_position()
    }

    pub fn get_materials(&self) -> i32 {
        self.entity.get_materials()
    }

    pub fn get_sprite(&self) -> char {
        self.sprite
    }

    pub fn get_light(&self) -> Light {
        Light::new(self.light.get_brightness(), self.get_position())
    }
}
