use super::{player, enemy, spawner, position};
use crate::{dungeon::{dungeon, room}, lighting::{lighting_manager, light::Light}};
use rand::seq::SliceRandom;


pub struct EntityManager {
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
    pub spawner: spawner::Spawner,
}


impl EntityManager {
    pub fn new(
        player: player::Player, 
        enemies: Vec<enemy::Enemy>, 
        turns_till_spawn: i32
    ) -> EntityManager {
        EntityManager { 
            player, 
            enemies, 
            spawner: spawner::Spawner::new(turns_till_spawn),
        }
    }


    pub fn spawn_enemy(&mut self, dungeon: dungeon::Dungeon) -> dungeon::Dungeon {
        let positions = dungeon.get_empty_rooms();
        let mut valid_positions : Vec<room::Room> = Vec::new();
        for i in 0..positions.len() {
            if positions[i].is_lighted == false {
                valid_positions.push(positions[i]);
            }
        }

        if valid_positions.is_empty() == true {
            return dungeon;
        }
        let room = valid_positions.choose(&mut rand::thread_rng()).unwrap();
        let mut enemy = vec![enemy::Enemy::new(room.position)];
        self.enemies.append(&mut enemy);
        dungeon
    }
        

    pub fn manage(&mut self, dungeon : dungeon::Dungeon) -> dungeon::Dungeon {
        let mut new_dungeon = dungeon;
        loop {
            let lights = self.get_lights(&mut new_dungeon);
            new_dungeon.rooms = lighting_manager::manage_light(&lights, &new_dungeon);
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
            if self.enemies[enemy].get_position() == self.player.get_position() {
                return true;
            }
        }
        return false;
    }


    fn manage_enemies(&mut self, dungeon: &dungeon::Dungeon) {
        let mut new_enemies = self.kill_enemies(dungeon);
        for i in 0..new_enemies.len() {
            new_enemies[i].do_action(&dungeon, &self.player, &self);
        }
        self.enemies = new_enemies;
    }


    fn kill_enemies(&mut self, dungeon: &dungeon::Dungeon) -> Vec<enemy::Enemy> {
        let mut new_enemies = self.enemies.clone();
        for enemy in 0..self.enemies.len() {
            if self.is_enemy_dead(enemy, dungeon) == true {
                new_enemies.remove(enemy);
            }
        }
        new_enemies
    }


    fn is_enemy_dead(
        &mut self, 
        enemy: usize, 
        new_dungeon: &dungeon::Dungeon
    ) -> bool {
        new_dungeon.does_position_have_collision(self.enemies[enemy].get_position())
    }


    pub fn get_enemy_positions(&self) -> Vec<&position::Position> {
        let mut positions = Vec::new();

        for enemy in 0..self.enemies.len() {
            positions.push(self.enemies[enemy].get_position());
        }

        positions
    }


    fn get_lights(&mut self, dungeon: &mut dungeon::Dungeon) -> Vec<Light> {
        let mut lights = vec![self.player.get_light()];
        lights.append(&mut dungeon.lights.clone());
        lights
    }


    pub fn does_position_have_collision(&self, pos: &position::Position) -> bool {
        for i in 0..self.enemies.len() {
            if self.enemies[i].get_position() == pos {
                return true;
            }
        }
        false
    }
}
