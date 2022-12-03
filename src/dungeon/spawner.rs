use serde_derive::{Serialize, Deserialize};

const SPAWN_SPEED : i32 = 15;


#[derive(Serialize, Deserialize, Debug)]
pub struct Spawner {
    pub turns_till_spawn: i32,
}

impl Spawner {
    pub fn new(turns_till_spawn: i32) -> Spawner {
        Spawner { turns_till_spawn }
    }


    pub fn should_spawn(&mut self) -> bool {
        if self.turns_till_spawn <= 0 {
            self.turns_till_spawn = SPAWN_SPEED;
            println!("turns till spawn: {}", self.turns_till_spawn);
            return true;
        } else {
            self.turns_till_spawn -= 1;
        }
        println!("turns till spawn: {}", self.turns_till_spawn);
        return false;
    }
}
