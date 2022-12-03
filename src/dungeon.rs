use std::io::{self,Read};

pub mod position;
pub mod dungeon;
pub mod player;
pub mod room;
pub mod walker; 
pub mod pathfinding;
pub mod entity_manager;
pub mod enemy;
pub mod spawner;
pub mod saver_loader;

pub fn play() {
    println!("use save? <y/n>");
    if get_input() == 'y' {
        play_with_save();
    } else {
        play_with_new_game();
    }
}


fn get_input() -> char {
    for byte in io::stdin().bytes() {
        let action = byte.unwrap() as char;
        return action;
    }
    return 'x'
}


fn play_with_save() {
    println!("paste save data please");

    let loader = saver_loader::Saver::load();

    let mut dungeon = loader.dungeon;
    let mut entity_manager = loader.manager;
    dungeon = entity_manager.manage(dungeon);

    saver_loader::Saver::save(dungeon, entity_manager);
}


fn play_with_new_game() {
    let mut dungeon = dungeon::Dungeon::new();
    let mut walker = walker::Walker::new();
    let player = player::Player::new(position::STOP, 0);
    let mut entity_manager = entity_manager::EntityManager::new(player, Vec::new(), 10);
    dungeon = walker.generate(300, dungeon);
    dungeon = entity_manager.spawn_enemy(dungeon);
    dungeon = entity_manager.manage(dungeon);

    saver_loader::Saver::save(dungeon, entity_manager);
}
