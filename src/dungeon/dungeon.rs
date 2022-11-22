use std::collections::HashMap;

use crate::dungeon::position;
use crate::dungeon::room;
use crate::dungeon::player;

use super::enemy;

const WINDOW_WIDTH: i32 = 20;
const WINDOW_HEIGHT: i32 = 13;

pub struct Dungeon {
    rooms: HashMap<position::Position, room::Room>,
    pub valid: bool,
}


impl Dungeon {
    pub fn new() -> Dungeon { 
        Dungeon {rooms: Self::make_empty_dungeon(), valid: true}
    }
        

    fn make_empty_dungeon() -> HashMap<position::Position, room::Room> {
        return HashMap::new();
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
        

    pub fn print(&self, player: &player::Player, enemies: &Vec<enemy::Enemy>) {
        let pos = player.get_position();
        for x in pos.x-WINDOW_HEIGHT..pos.x+WINDOW_HEIGHT {
            for y in pos.y-WINDOW_WIDTH..pos.y+WINDOW_WIDTH {
                let index = position::Position::new(x, y);
                let mut art = '#';
                
                if self.rooms.contains_key(&index) {
                    art = self.rooms[&index].art;
                }

                if player.get_position() == &index {
                    art = player.sprite;
                }

                for enemy in 0..enemies.len() {
                    if enemies[enemy].get_position() == &index {
                        art = enemies[enemy].sprite;
                    }
                }

                print!("{} ", art);
            }
            println!();
        }
    }


    pub fn does_position_have_collision(&self, pos : &position::Position) -> bool {
        !self.rooms.contains_key(pos)
    }


    pub fn get_empty_rooms(&self) -> Vec<room::Room> {
        self.rooms.values().cloned().collect::<Vec<room::Room>>()
    }


    pub fn get_map_x(&self) -> Vec<i32> {

        let mut x = Vec::new();
        for (_, position) in &self.rooms {
            x.append(&mut vec![position.position.x]);
        }
        x
    }


    pub fn get_map_y(&self) -> Vec<i32> {
        let mut y = Vec::new();
        for (_, position) in &self.rooms {
            y.append(&mut vec![position.position.y]);
        }
        y
    }

}
