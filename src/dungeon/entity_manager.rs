use super::{player, dungeon, enemy, spawner, position};
use rand::seq::SliceRandom;


pub struct EntityManager {
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
    pub spawner: spawner::Spawner,
}


impl EntityManager {
    pub fn new(player: player::Player, enemies: Vec<enemy::Enemy>, turns_till_spawn: i32) -> EntityManager {
        EntityManager { player, enemies, spawner: spawner::Spawner::new(turns_till_spawn)}
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

            if self.check_for_loss() {
                println!("You Lose!");
                break;
            }

            new_dungeon = self.player.do_action(new_dungeon);

            if new_dungeon.valid == false {
                break;
            }
            
            self.manage_enemies(&new_dungeon);

            if self.spawner.should_spawn() {
                new_dungeon = self.spawn_enemy(new_dungeon)
            }
        }
        new_dungeon
    }
     

    fn check_for_loss(&self) -> bool {
        for enemy in 0..self.enemies.len() {
            if self.enemies[enemy].unit.position == self.player.get_position() {
                return true;
            }
        }
        return false;
    }


    fn manage_enemies(&mut self, dungeon: &dungeon::Dungeon) {
        for enemy in 0..self.enemies.len() {
            if self.enemies.len() <= 0 { break; }
            self.is_enemy_dead(enemy, &dungeon);
            if self.enemies.len() <= 0 { break; }
            self.enemies[enemy].do_action(&dungeon, &self.player);
        }
    }


    fn is_enemy_dead(&mut self, enemy: usize, new_dungeon: &dungeon::Dungeon) {
        if new_dungeon.does_position_have_collision(
            &self.enemies[enemy].unit.position
        ) {
            self.enemies.remove(enemy);
        }
    }

    pub fn get_enemy_positions(&self) -> Vec<&position::Position> {
        let mut positions = Vec::new();

        for enemy in 0..self.enemies.len() {
            positions.append(&mut vec![&self.enemies[enemy].unit.position]);
        }

        positions
    }
}
