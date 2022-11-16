use std::collections::HashMap;
use crate::dungeon::position;
use crate::dungeon::room;

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
        

    pub fn print_with_player(&self, pos: &position::Position, sprite: &char) {
        for x in pos.x-20..pos.x+20 {
            for y in pos.y-20..pos.y+20 {
                let index = position::Position::new(x, y);
                if &index == pos {
                    print!("{} ", sprite)
                }  else if self.rooms.contains_key(&index) {
                    let art = self.rooms[&index].art;
                    print!("{} ", art);
                } else {
                    print!("# ");
                }
            }
            println!();
        }
    }


    pub fn does_position_have_collision(&self, pos : &position::Position) -> bool {
        !self.rooms.contains_key(pos)
    }

}
