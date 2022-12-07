use std::collections::HashMap;

use crate::entities::position;
use crate::dungeon::room;
use crate::entities::player;
use crate::entities::enemy;
use crate::entities::robot;
use crate::lighting::light::Light;

const WINDOW_WIDTH: i32 = 20;
const WINDOW_HEIGHT: i32 = 13;
const UNSEEN_ROOM_ART: char = '#';


pub struct Dungeon {
    rooms: HashMap<position::Position, room::Room>,
    pub valid: bool,
    pub lights: Vec<Light>,
    pub robots: Vec<robot::Robot>,
}


impl Dungeon {
    pub fn new() -> Dungeon { 
        Dungeon {rooms: Self::make_empty_dungeon(), valid: true, lights: Vec::new(), robots: Vec::new() }
    }
        

    fn make_empty_dungeon() -> HashMap<position::Position, room::Room> {
        return HashMap::new();
    }

    pub fn does_position_have_collision(&self, pos : &position::Position) -> bool {
        !self.rooms.contains_key(pos)
    }


    pub fn get_empty_rooms(&self) -> Vec<room::Room> {
        self.rooms.values().cloned().collect::<Vec<room::Room>>()
    }


    pub fn get_robots(&self) -> &Vec<robot::Robot> {
        &self.robots
    }

    pub fn add_to_robot_pattern_at_position(&mut self, pos: &position::Position, action: char) {
        for i in 0..self.robots.len() {
            if &self.robots[i].get_position() == pos {
                self.robots[i].add_to_pattern(action);
                return;
            }
        }
        println!("ROBOT NOT FOUND! at {:?}", pos);
    }

    pub fn manage_robots(&mut self) {
        let mut new_robots = self.robots.clone();
        for i in 0..new_robots.len() {
            new_robots[i].do_action(self)
        }
        self.robots = new_robots;
    }


    pub fn light_or_unlight_room_at_position(&mut self, should_light: bool, pos: &position::Position) {
        if self.rooms.contains_key(pos) == true {
            self.rooms.get_mut(pos).unwrap().is_lighted = should_light;
        }
    }

    pub fn load_map(&mut self, map : Vec<room::Room>) {
        for room in map {
            self.rooms.insert(room.position.clone(), room);
        }
    }


    pub fn set_room(&mut self, pos: &position::Position, room: room::Room) {
        self.rooms.entry(*pos).and_modify(|index| *index = room).or_insert(room);
    }


    pub fn erase_room(&mut self, pos: &position::Position) {
        self.rooms.remove(pos);
    }


    pub fn set_room_as_empty(&mut self, pos: position::Position) {
        self.set_room(&pos, room::Room::new('.', pos, 0))
    }
        

    pub fn print(&self, player: &player::Player, enemies: &Vec<enemy::Enemy>) {
        let pos = player.get_position();
        for x in pos.get_x()-WINDOW_HEIGHT..pos.get_x()+WINDOW_HEIGHT {
            for y in pos.get_y()-WINDOW_WIDTH..pos.get_y()+WINDOW_WIDTH {

                let index = position::Position::new(x, y);
                let mut art = UNSEEN_ROOM_ART;
                
                art = self.should_be_room(art, &index);
                art = self.should_be_light(art, &index);
                art = self.should_be_robot(art, &index);
                art = self.should_be_player(art, &index, player);
                art = self.should_be_enemy(art, &index, enemies);
                art = self.should_be_unlit(art, &index);

                print!("{} ", art);
            }
            println!();
        }
    }

    fn should_be_room(&self, old_art: char, index: &position::Position) -> char {
        if self.rooms.contains_key(index) {
            return self.rooms[index].art;
        }
        return old_art;
    }

    fn should_be_light(&self, old_art: char, index: &position::Position) -> char {
        for i in 0..self.lights.len() {
            if self.lights[i].get_position() == index {
                return 'i';
            }
        }
        return old_art;
    }

    fn should_be_player(&self, old_art: char, index: &position::Position, player: &player::Player) -> char {
        if &player.get_position() == index {
            return player.get_sprite();
        }
        return old_art;
    }

    fn should_be_enemy(&self, old_art: char, index: &position::Position, enemies: &Vec<enemy::Enemy>) -> char {
        for enemy in 0..enemies.len() {
            if &enemies[enemy].get_position() == &index {
                return enemies[enemy].sprite;
            }
        }
        return old_art;
    }


    fn should_be_robot(&self, old_art: char, index: &position::Position) -> char {
        for i in self.get_robots() {
            if &i.get_position() == index {
                return i.get_sprite();
            }
        }
        return old_art;
    }

    fn should_be_unlit(&self, old_art: char, index: &position::Position) -> char {
        if self.rooms.contains_key(&index) {
            if self.rooms[&index].is_lighted == false {
                return UNSEEN_ROOM_ART;
            }
        }
        return old_art;
    }

}
