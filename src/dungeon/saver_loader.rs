use crate::dungeon::dungeon;
use crate::dungeon::entity_manager;
use text_io::read;
use serde_derive::{Serialize, Deserialize};

use super::room::Room;
use super::position::Position;
use super::dungeon::Dungeon;
use super::entity_manager::EntityManager;
use super::player::Player;
use super::enemy::Enemy;


#[derive(Serialize, Deserialize, Debug)]
pub struct Saver {
    pub dungeon_map_x: Vec<i32>,
    pub dungeon_map_y: Vec<i32>,
    pub player_pos_x: i32,
    pub player_pos_y: i32,
    pub player_materials: i32,
    pub enemy_positions_x: Vec<i32>,
    pub enemy_positions_y: Vec<i32>,
    pub turns_till_spawn: i32,
}


pub struct Loader {
    pub dungeon: Dungeon,
    pub manager: EntityManager,
}


impl Saver {
    pub fn save(
        dungeon: dungeon::Dungeon, 
        manager: entity_manager::EntityManager
    ) {
        let saver = Saver {   
            dungeon_map_x: dungeon.get_map_x(), 
            dungeon_map_y: dungeon.get_map_y(), 
            player_pos_x: manager.player.get_position().x,
            player_pos_y: manager.player.get_position().y,
            player_materials: manager.player.get_materials(),
            enemy_positions_x: get_enemy_positions_x(&manager),
            enemy_positions_y: get_enemy_positions_y(&manager),
            turns_till_spawn: manager.spawner.turns_till_spawn,
        };
        let serialized = serde_json::to_string(&saver).unwrap();

        println!("Game save: {}", serialized);
    }

    pub fn load() -> Loader {
        let input : String = read!();
        let str_input : &str = &input[..];
        // Convert the JSON string back to a Point.
        let deserialized: Saver = serde_json::from_str(str_input).unwrap();
        Loader::load(deserialized)
    }
}


impl Loader {
    fn load(saver: Saver) -> Loader {

        let dungeon = Loader::load_dungeon(&saver);
        let player = Player::new(
            Position::new(saver.player_pos_x, saver.player_pos_y), 
            saver.player_materials
        );

        let mut enemies = Vec::new();

        for enemy in 0..saver.enemy_positions_x.len() {
            enemies.append(&mut vec![
                Enemy::new(
                    Position::new(
                        saver.enemy_positions_x[enemy], 
                        saver.enemy_positions_y[enemy]
                    )
                    
                )
                
            ])
        }

        let manager = EntityManager::new(player, enemies, saver.turns_till_spawn);

        Loader { dungeon, manager }

    }

    
    fn load_dungeon(saver: &Saver) -> Dungeon {
        let mut dungeon = Dungeon::new();
        let mut dungeon_map_pos = Vec::new();
        let mut map = Vec::new();
        for index in 0..saver.dungeon_map_x.len() {
            dungeon_map_pos.append(
                &mut vec![Position::new(
                    saver.dungeon_map_x[index], 
                    saver.dungeon_map_y[index]
                )]
            )
        }

        for pos in dungeon_map_pos {
            map.append(&mut vec![Room::new('.', pos)])
        }
        dungeon.load_map(map);
        dungeon
    }
}


fn get_enemy_positions_x(manager : &EntityManager) -> Vec<i32> {
    let mut x = Vec::new();
    let pos = manager.get_enemy_positions();

    for i in pos {
        x.append(&mut vec![i.x]);
    }

    x
}


fn get_enemy_positions_y(manager : &EntityManager) -> Vec<i32> {
    let mut y = Vec::new();
    let pos = manager.get_enemy_positions();

    for i in pos {
        y.append(&mut vec![i.y]);
    }

    y
}
