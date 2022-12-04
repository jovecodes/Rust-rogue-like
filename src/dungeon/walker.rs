use crate::dungeon::dungeon;
use rand::Rng;
use crate::entities::position;

use super::room;


pub struct Walker {
    position: position::Position,
    steps_since_turn: i8,
    current_direction: position::Position,
}


impl Walker {
    pub fn new() -> Walker { 
        Walker {
            position            : position::Position::new(0, 0),
            steps_since_turn    : 0,
            current_direction   : position::STOP,
        }
    }


    pub fn generate(&mut self, steps: i32, dungeon: dungeon::Dungeon) -> dungeon::Dungeon {
        let mut new_dungeon = dungeon;
        self.current_direction = self.get_turn();
        for _ in 0..steps {
            new_dungeon = self.mine(new_dungeon);
            self.step();
            self.turn();
        }

        new_dungeon = self.place_goal(new_dungeon);
        new_dungeon
    }


    fn mine(&self, mut dungeon: dungeon::Dungeon) -> dungeon::Dungeon {
       dungeon.set_room(&self.position, room::Room::new('.', self.position, 0)); 
       dungeon
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
    

    fn place_goal(&mut self, mut dungeon: dungeon::Dungeon) -> dungeon::Dungeon{
        self.step(); 
        dungeon.set_room(&self.position, room::Room::new('G', self.position, 0)); 
        dungeon
    }
    
}
