mod dungeon;
use dungeon::{position, walker};

fn main() {
    make_dungeon();
}

fn make_dungeon() {
    let mut dungeon = dungeon::dungeon::Dungeon::new();
    let mut walker = walker::Walker::new();
    let mut player = dungeon::player::Player::new(position::STOP);
    dungeon = walker.generate(300, dungeon);
    loop {
        player.look(&dungeon);
        if player.do_action(&dungeon) == false {
            break;
        }
    }
}
