use rand::{self, Rng};

const EMPTY_ROOM:      Room = Room{art: '#', left: false, right: false, down: false, up: false};
const OPEN_ROOM:       Room = Room{art: '.', left: true,  right: true,  down: true,  up: true };
const GOAL_ROOM:       Room = Room{art: 'G', left: true,  right: true,  down: true,  up: true };

const HORIZONTAL_ROOM: Room = Room{art: '=', left: true,  right: true,  down: false, up: false};
const VERTICLE_ROOM:   Room = Room{art: '|', left: false, right: false, down: true,  up: true };

const RIGHT_ROOM:      Room = Room{art: '[', left: false, right: true,  down: false, up: false};
const LEFT_ROOM:       Room = Room{art: ']', left: true,  right: false, down: false, up: false};
const DOWN_ROOM:       Room = Room{art: 'M', left: false, right: false, down: true,  up: false};
const UP_ROOM:         Room = Room{art: 'U', left: false, right: false, down: false, up: true };


#[derive(Clone, Copy, Debug)]
struct Room {
    art: char,
    left: bool,
    right: bool,
    down: bool,
    up: bool,
}

pub struct Dungeon {
    rooms: Vec<Room>,
    size_x: i32,
    size_y: i32,
}


impl Dungeon {
    pub fn new(x: i32, y: i32) -> Dungeon { 
        Dungeon {size_x : x, size_y : y, rooms: Self::make_empty_dungeon(x, y)}
    }
        

    pub fn pos_to_index(&self, x: i32, y: i32) -> i32 {
        return self.size_x * y + x;
    }


    fn make_empty_dungeon(x: i32, y: i32) -> Vec<Room> {
        return vec![EMPTY_ROOM; (x*y).try_into().unwrap()];
    }


    pub fn set_empty(&mut self, pos: usize) {
        if pos > self.rooms.len() {
            return;
        }
        self.rooms[pos] = OPEN_ROOM;
    }
        

    pub fn set_goal(&mut self, pos: usize) {
        if pos > self.rooms.len() {
            return;
        }
        self.rooms[pos] = GOAL_ROOM;
    }


    pub fn get_x(&self) -> i32 {
        self.size_x
    }
        

    pub fn get_y(&self) -> i32 {
        self.size_y
    }
        

    pub fn print(&self) {
        let mut index = 0;
        for _ in 0..self.size_x {
            for _ in 0..self.size_x {
                print!("{} ", self.rooms[index].art);
                index += 1;
            }
            print!("\n");
        }
    }
}
