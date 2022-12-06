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

    
    pub fn manage(&mut self, dungeon : &mut dungeon::Dungeon) {
        self.set_lighting(dungeon);
        loop {
            dungeon.print(&self.player, &self.enemies);

            self.player.do_action(dungeon);
            self.set_lighting(dungeon);
            self.manage_enemies(&dungeon);
            self.spawn(dungeon);

            if self.should_lose(&dungeon) {
                break;
            }
        }
        dungeon.print(&self.player, &self.enemies);
    }


    fn spawn(&mut self, dungeon: &mut dungeon::Dungeon) {
        if self.spawner.should_spawn() {
            self.spawn_enemy(dungeon);
        }
    }

    pub fn spawn_enemy(&mut self, dungeon: &mut dungeon::Dungeon) {
        let valid_positions = EntityManager::get_spawnable_rooms(dungeon);
        if valid_positions.is_empty() == true {
            return;
        }
        let room = valid_positions.choose(&mut rand::thread_rng()).unwrap();
        let mut enemy = vec![enemy::Enemy::new(room.position)];
        self.enemies.append(&mut enemy);
    }


    fn get_spawnable_rooms(dungeon: &mut dungeon::Dungeon) -> Vec<room::Room>{
        let rooms = dungeon.get_empty_rooms();
        let mut valid_rooms : Vec<room::Room> = Vec::new();
        for i in 0..rooms.len() {
            if rooms[i].is_lighted == false {
                valid_rooms.push(rooms[i]);
            }
        }
        valid_rooms
    }
        

    fn should_lose(&self, dungeon: &dungeon::Dungeon) -> bool {
        return dungeon.valid == false || self.check_for_loss() == true;
    }

    fn check_for_loss(&self) -> bool {
        for enemy in 0..self.enemies.len() {
            if self.enemies[enemy].get_position() == self.player.get_position() {
                println!("You Lose!");
                return true;
            }
        }
        return false;
    }

    pub fn get_enemy_positions(&self) -> Vec<&position::Position> {
        let mut positions = Vec::new();

        for enemy in 0..self.enemies.len() {
            positions.push(self.enemies[enemy].get_position());
        }

        positions
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
        dungeon: &dungeon::Dungeon
    ) -> bool {
        dungeon.does_position_have_collision(self.enemies[enemy].get_position())
    }



    fn set_lighting(&mut self, dungeon: &mut dungeon::Dungeon) {
        let lights = self.get_lights(dungeon);
        lighting_manager::manage_light(&lights, dungeon);
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
