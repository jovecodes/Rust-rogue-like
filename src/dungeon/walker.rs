use crate::dungeon::dungeon;
use rand::Rng;
use crate::dungeon::position;

use super::room;


pub struct Walker {
    position: position::Position,
    dungeon: dungeon::Dungeon,
    steps_since_turn: i8,
    current_direction: position::Position,
}


impl Walker {
    pub fn new() -> Walker { 
        Walker {
            position            : position::Position::new(0, 0),
            steps_since_turn    : 0,
            dungeon             : dungeon::Dungeon::new(),
            current_direction   : position::STOP,
        }
    }


    pub fn generate(&mut self, steps: i32) {
        self.current_direction = self.get_turn();
        for _ in 0..steps {
            self.mine();
            self.step();
            self.turn();
        }

        self.place_goal();

    }


    fn mine(&mut self) {
       self.dungeon.set_room(&self.position, room::OPEN_ROOM); 
    }


    fn step(&mut self){
        self.position.x += self.current_direction.x;
        self.position.y += self.current_direction.y;
        self.steps_since_turn += 1;
    }


    fn turn(&mut self) {
        self.current_direction = self.get_turn();
        self.steps_since_turn = 0;
    }
        

    fn get_turn(&mut self) -> position::Position {
        let mut rng = rand::thread_rng();
        return self.current_direction.set_with_int(rng.gen_range(0..4));
    }
    

    fn place_goal(&mut self){
        self.step(); 
        self.dungeon.set_room(&self.position, room::GOAL_ROOM); 
    }


    pub fn print(&self) {
       self.dungeon.print(&self.position); 
    }

}
