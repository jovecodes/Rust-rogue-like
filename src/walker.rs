use super::dungeon::Dungeon;
use rand::Rng;

const LEFT  : Position = Position {x:-1, y: 0};
const RIGHT : Position = Position {x: 1, y: 0};
const DOWN  : Position = Position {x: 0, y: 1};
const UP    : Position = Position {x: 0, y:-1};
const STOP  : Position = Position {x: 0, y: 0};

pub struct Walker {
    position: Position,
    dungeon: Dungeon,
    steps_since_turn: i8,
    max_steps_sice_turn: i8,
    chance_to_turn: f32,
    current_direction: Position,
}

struct Position {x: i32, y: i32}

impl Position {
    //fn add(&mut self, pos : Position) {
    //    self.x += pos.x;
    //    self.y += pos.y;
    //}
    
    //fn set(&mut self, pos: Position) {
    //    self.x = pos.x;
    //    self.y = pos.y;
    //}


    fn equals(&self, pos: &Position) -> bool {
        return self.x == pos.x && self.y == pos.y;
    }


    fn copy(&self) -> Position {
        return Position { x: self.x, y: self.y }
    }


    fn set_with_int(&mut self, int : i32) -> Position {
        match int {
            0 => return LEFT,
            1 => return RIGHT,
            2 => return DOWN,
            3 => return UP,
            _ => return STOP
        }
    }
}

impl Walker {
    pub fn new(x: i32, y: i32, chance: f32, max_steps: i8) -> Walker { 
        Walker {
            position            : Position { x: x / 2, y: y / 2 },
            steps_since_turn    : 0,
            max_steps_sice_turn : max_steps,
            dungeon             : Dungeon::new(x, y),
            chance_to_turn      : chance,
            current_direction   : STOP,
        }
    }


    pub fn generate(&mut self, steps: i32) {
        self.current_direction = self.get_turn();
        for _ in 0..steps {
            self.mine();
            self.step();
            self.turn(false);
        }

        self.place_goal();

        if false {
            self.dungeon.generate();
        }
    }


    fn mine(&mut self) {
       self.dungeon.set_empty(self.dungeon.pos_to_index(self.position.x, self.position.y) as usize); 
    }


    fn step(&mut self){
        if (self.position.x + self.current_direction.x) >= self.dungeon.get_x() {
            self.turn(true);
            println!("i was going to go out of bound");
        }
        if (self.position.y + self.current_direction.y + 2) >= self.dungeon.get_y() {
            self.turn(true);
            println!("i was going to go out of bound");
        }
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) < self.chance_to_turn {
            self.turn(false)
        }

        self.position.x += self.current_direction.x;
        self.position.y += self.current_direction.y;
        self.steps_since_turn += 1;
    }


    fn turn(&mut self, force: bool) {
        if self.steps_since_turn >= self.max_steps_sice_turn {
            let last_current = &self.current_direction.copy();
            self.current_direction = self.get_turn();
            if force {
                while self.current_direction.equals(last_current) {
                    self.current_direction = self.get_turn();
                }
            }
        }
    }
        

    fn get_turn(&mut self) -> Position {
        let mut rng = rand::thread_rng();
        return self.current_direction.set_with_int(rng.gen_range(0..4));
    }
    
    fn place_goal(&mut self){
        self.step(); 
       self.dungeon.set_goal(self.dungeon.pos_to_index(self.position.x, self.position.y) as usize); 
    }

    pub fn print(&self) {
       self.dungeon.print(); 
    }

}
