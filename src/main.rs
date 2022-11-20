mod dungeon;
use dungeon::{position, walker, entity_manager};

fn main() {
    make_dungeon();
}

fn make_dungeon() {
    let mut dungeon = dungeon::dungeon::Dungeon::new();
    let mut walker = walker::Walker::new();
    let player = dungeon::player::Player::new(position::STOP);
    let mut entity_manager = entity_manager::EntityManager::new(player);
    dungeon = walker.generate(300, dungeon);
    dungeon = entity_manager.spawn_enemy(dungeon);
    entity_manager.manage(dungeon);
}
