use crate::dungeon::dungeon;
use crate::entities::entity_manager;
use crate::lighting::light::Light;
use text_io::read;
use serde_derive::{Serialize, Deserialize};
use magic_crypt::MagicCryptTrait;

use crate::dungeon::room::Room;
use crate::entities::position::Position;
use crate::dungeon::dungeon::Dungeon;
use crate::entities::entity_manager::EntityManager;
use crate::entities::player::Player;
use super::entities::enemy::Enemy;


#[derive(Serialize, Deserialize, Debug)]
struct Pos {
    x: i32,
    y: i32,
}


impl Pos {
   pub fn to_position(&self) -> Position {
       Position::new(self.x, self.y)
   }

   pub fn from_position(pos: &Position) -> Pos {
       Pos {x: pos.get_x(), y: pos.get_y()}
   }

   pub fn position_vec_to_pos_vec(positions: Vec<&Position>) -> Vec<Pos> {
       let mut pos_vec = Vec::new();
       for pos in positions {
           pos_vec.push(Pos::from_position(pos));
       }
       pos_vec
   }

   pub fn pos_vec_to_position_vec(positions: &Vec<Pos>) -> Vec<Position> {
       let mut pos_vec = Vec::new();
       for pos in positions {
           pos_vec.push(Pos::to_position(&pos));
       }
       pos_vec
   }

   pub fn room_vec_to_pos_vec(rooms: Vec<Room>) -> Vec<Pos> {
       let mut pos_vec = Vec::new();
       for i in rooms {
            pos_vec.push(Pos::from_position(&i.position));
       }
       pos_vec
   }

   pub fn light_vec_to_pos_vec(rooms: &Vec<Light>) -> Vec<Pos> {
       let mut pos_vec = Vec::new();
       for i in rooms {
            pos_vec.push(Pos::from_position(&i.get_position()));
       }
       pos_vec
   }

   pub fn pos_vec_to_light_vec(positions: &Vec<Pos>) -> Vec<Light> {
       let mut pos_vec = Vec::new();
       for pos in positions {
           pos_vec.push(Light::new(10, Pos::to_position(&pos)));
       }
       pos_vec
   }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Saver {
    dungeon_map: Vec<Pos>,
    player_pos: Pos,
    player_materials: i32,
    enemy_positions: Vec<Pos>,
    turns_till_spawn: i32,
    light_positions: Vec<Pos>,
}


pub struct Loader {
    pub dungeon: Dungeon,
    pub manager: EntityManager,
}


impl Saver {
    pub fn save(
        dungeon: dungeon::Dungeon, 
        manager: entity_manager::EntityManager,
    ) {
        let saver = Saver {   
            dungeon_map: Pos::room_vec_to_pos_vec(dungeon.get_empty_rooms()),
            player_pos: Pos::from_position(manager.player.get_position()),
            player_materials: manager.player.get_materials(),
            enemy_positions: Pos::position_vec_to_pos_vec(manager.get_enemy_positions()),
            turns_till_spawn: manager.spawner.turns_till_spawn,
            light_positions: Pos::light_vec_to_pos_vec(&dungeon.lights)
        };
        let mcrypt = new_magic_crypt!("magickey", 256);
        let serialized = serde_json::to_string(&saver).unwrap();

        let encrypted_save = mcrypt.encrypt_str_to_base64(serialized);

        println!("Game save: {}", encrypted_save);
    }

    pub fn load() -> Loader {
        let mcrypt = new_magic_crypt!("magickey", 256);

        let input : String = read!();
        let decypted_save = mcrypt.decrypt_base64_to_string(&input).unwrap();
        let str_input : &str = &decypted_save[..];
        // Convert the JSON string back to a Point.
        let deserialized: Saver = serde_json::from_str(str_input).unwrap();
        Loader::load(deserialized)
    }
}


impl Loader {
    fn load(saver: Saver) -> Loader {

        let dungeon = Loader::load_dungeon(&saver);
        let player = Player::new(
            saver.player_pos.to_position(), 
            saver.player_materials
        );

        let mut enemies = Vec::new();

        for enemy in 0..saver.enemy_positions.len() {
            enemies.push(Enemy::new(saver.enemy_positions[enemy].to_position()))
        }

        let manager = EntityManager::new(player, enemies, saver.turns_till_spawn);

        Loader { dungeon, manager }

    }

    
    fn load_dungeon(saver: &Saver) -> Dungeon {
        let mut dungeon = Dungeon::new();
        let dungeon_map_positions = Pos::pos_vec_to_position_vec(&saver.dungeon_map);
        let mut map = Vec::new();

        for pos in dungeon_map_positions {
            map.push(Room::new('.', pos, 0))
        }

        dungeon.load_map(map);
        dungeon.lights.append(&mut Pos::pos_vec_to_light_vec(&saver.light_positions));
        dungeon
    }
}
