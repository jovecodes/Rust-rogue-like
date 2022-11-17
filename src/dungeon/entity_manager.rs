use super::{player, dungeon, enemy};
use rand::seq::SliceRandom;

pub struct EntityManager {
    player: player::Player,
    enemies: Vec<enemy::Enemy>,
}


impl EntityManager {
    pub fn new(player: player::Player) -> EntityManager {
        EntityManager { player, enemies: Vec::new() }
    }


    pub fn spawn_enemy(&mut self, dungeon: dungeon::Dungeon) -> dungeon::Dungeon {
        let positions = dungeon.get_empty_rooms();
        let room = positions.choose(&mut rand::thread_rng()).unwrap();
        let mut enemy = vec![enemy::Enemy::new(room.position)];
        self.enemies.append(&mut enemy);
        dungeon
    }
        

    pub fn manage(&mut self, dungeon : &dungeon::Dungeon) {
        loop {
            dungeon.print(&self.player, &self.enemies);
            if self.player.do_action(dungeon) == false {
                break;
            }
            for enemy in 0..self.enemies.len() {
                self.enemies[enemy].do_action(dungeon);
            }
        }

    }
}
