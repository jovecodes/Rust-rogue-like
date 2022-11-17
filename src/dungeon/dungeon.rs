use std::collections::HashMap;
use crate::dungeon::position;
use crate::dungeon::room;
use crate::dungeon::player;

use super::enemy;

const WINDOW_WIDTH: i32 = 20;
const WINDOW_HEIGHT: i32 = 14;

#[derive(Clone, Eq, PartialEq)]
pub struct Dungeon {
    rooms: HashMap<position::Position, room::Room>,
}


impl Dungeon {
    pub fn new() -> Dungeon { 
        Dungeon {rooms: Self::make_empty_dungeon()}
    }
        

    fn make_empty_dungeon() -> HashMap<position::Position, room::Room> {
        return HashMap::new();
    }


    pub fn set_room(&mut self, pos: &position::Position, room: room::Room) {
        self.rooms.entry(*pos).and_modify(|index| *index = room).or_insert(room);
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

                for enemy in 0..enemies.len() {
                    if enemies[enemy].unit.position == index {
                        art = enemies[enemy].unit.sprite;
                    }
                }

                if player.get_position() == index {
                    art = player.unit.sprite;
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

}
