
pub struct Spawner {
    turns_till_spawn: i32,
}

impl Spawner {
    pub fn new() -> Spawner {
        Spawner { turns_till_spawn: 10 }
    }


    pub fn should_spawn(&mut self) -> bool {
        if self.turns_till_spawn <= 0 {
            self.turns_till_spawn = 10;
            println!("turns till spawn: {}", self.turns_till_spawn);
            return true;
        } else {
            self.turns_till_spawn -= 1;
        }
        println!("turns till spawn: {}", self.turns_till_spawn);
        return false;
    }
}
