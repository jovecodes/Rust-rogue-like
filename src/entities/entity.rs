use crate::{dungeon::dungeon::Dungeon, lighting::light::Light};

use super::{position::{Position, self}, robot::Robot};


const MOVE_UP : char = 'w';
const MOVE_LEFT : char = 'a';
const MOVE_DOWN : char = 's';
const MOVE_RIGHT : char = 'd';
const MINE : char = 'm';
const PLACE_LIGHT : char = 'l';
const ROBOT_STUFF : char = 'r';
const BUILD : char = 'b';
const QUIT : char = 'q';

#[derive(Clone)]
pub struct Entity {
    action: Action,
    position: Position,
    materials: i32,
    robot_position: Position,
}

#[derive(Clone)]
enum Action {
    Walk,
    Build,
    Mine,
    PlaceLight,
    PlaceRobot,
    ConfigureRobot,
}


impl Entity {
    pub fn new(position: Position, materials: i32) -> Entity {
        Entity { action: Action::Walk, position, materials, robot_position : position::STOP }
    }

    pub fn get_materials(&self) -> i32 {
        self.materials
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn do_action(&mut self, action: char, dungeon: &mut Dungeon) -> bool {
        match self.action {
            Action::ConfigureRobot => self.configure_robot(action, dungeon),
            _ => return self.match_action(action, dungeon)
            
        }
        true
    }

    fn match_action(&mut self, action: char, dungeon: &mut Dungeon) -> bool {
        match &action {
            &MOVE_UP => self.get_action(position::LEFT, dungeon),
            &MOVE_LEFT => self.get_action(position::UP, dungeon),
            &MOVE_DOWN => self.get_action(position::RIGHT, dungeon),
            &MOVE_RIGHT => self.get_action(position::DOWN, dungeon),
            &MINE => self.action = Action::Mine,
            &BUILD => self.action = Action::Build,
            &PLACE_LIGHT => self.action = Action::PlaceLight,
            &ROBOT_STUFF => self.action = Action::PlaceRobot,
            &QUIT => {
                dungeon.valid = false;
            },
            _ => return false,
        }
        return true;
    }

    fn get_action(&mut self, direction: Position, dungeon: &mut Dungeon) {
        match self.action {
            Action::Walk => self.walk(direction, dungeon),
            Action::Build => self.build(direction, dungeon),
            Action::Mine => self.mine(direction, dungeon),
            Action::PlaceLight => self.place_light(direction, dungeon),
            Action::PlaceRobot => self.robot_stuff(direction, dungeon),
            _ => panic!("Invalid Action")
        }
    }

    fn walk(
        &mut self, direction: Position, 
        dungeon: &Dungeon
    ) {
       let future_position = self.position.get_add(&direction);
       if dungeon.does_position_have_collision(&future_position) == false {
            self.position.add(direction);
       }
    }


    fn mine(&mut self, direction: Position, dungeon: &mut Dungeon) {
        self.position.add(direction);

        if dungeon.does_position_have_collision(&self.position) {
            self.materials += 1;
        }

        dungeon.set_room_as_empty(self.position);
        self.action = Action::Walk;
    }


    fn build( &mut self, direction: Position, dungeon: &mut Dungeon) {
        if self.materials <= 0 {
            return;
        }

        let build_pos = &self.position.get_add(&direction);
        
        if dungeon.does_position_have_collision(&build_pos) == true {
            return;
        }

        self.materials -= 1;
        dungeon.erase_room(build_pos);
        self.action = Action::Walk;
    }


    fn robot_stuff(&mut self, direction: Position, dungeon: &mut Dungeon) {
        if self.should_place_robot(direction, dungeon) {
            self.place_robot(direction, dungeon);
        } else if self.should_configure_robot(direction, dungeon) {
            self.action = Action::ConfigureRobot;
        }
    }


    fn should_place_robot(&self, direction: Position, dungeon: &mut Dungeon) -> bool {
        let target_position = self.position.get_add(&direction);
        if dungeon.does_position_have_collision(&target_position) {
            return false;
        }
        for bot in dungeon.get_robots() {
            if bot.get_position() == target_position {
                return false;
            }
        }
        return true;
    }


    fn should_configure_robot(&mut self, direction: Position, dungeon: &mut Dungeon) -> bool {
        let future_position = self.position.get_add(&direction);
        for bot in dungeon.get_robots() {
            if bot.get_position() == future_position {
                self.robot_position = bot.get_position();
                return true;
            }
        }
        return false;
    }
    

    fn place_robot(&mut self, direction: Position, dungeon: &mut Dungeon) {
        let robot = Robot::new(self.position.get_add(&direction));
        dungeon.robots.push(robot);
        self.action = Action::Walk;
    }


    fn configure_robot(&mut self, action: char, dungeon: &mut Dungeon) {
        println!("ACTION: l{}l", action);
        dungeon.add_to_robot_pattern_at_position(&self.robot_position, action);
        self.action = Action::Walk;
    }


     
    fn place_light( &mut self, direction: Position, dungeon: &mut Dungeon) {
        dungeon.lights.push(self.make_light_in_direction(&direction));
        self.action = Action::Walk;
    }

    fn make_light_in_direction(&self, direction: &Position) -> Light {
        Light::new(10, self.position.get_add(direction))
    }
}


pub fn is_action_valid(action: &char) -> bool {
    match action {
        &MOVE_UP => true,
        &MOVE_LEFT => true,
        &MOVE_DOWN => true,
        &MOVE_RIGHT => true,
        &MINE => true,
        &BUILD => true,
        &PLACE_LIGHT => true,
        &ROBOT_STUFF => true,
        _ => false
    }
}
