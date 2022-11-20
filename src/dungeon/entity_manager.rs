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
        

    pub fn manage(&mut self, dungeon : dungeon::Dungeon) -> dungeon::Dungeon {
        let mut new_dungeon = dungeon;
        loop {
            new_dungeon.print(&self.player, &self.enemies);

            new_dungeon = self.player.do_action(new_dungeon);

            if new_dungeon.valid == false {
                break;
            }

            for enemy in 0..self.enemies.len() {
                self.enemies[enemy].do_action(&new_dungeon, &self.player);
            }

        }
        new_dungeon
    }
}
