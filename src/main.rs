mod dungeon;
use crate::walker::Walker;
mod walker;

fn main() {
    make_dungeon();
}

fn make_dungeon() {
    let mut dungeon = Walker::new(40, 40, 0.1, 5);
    dungeon.generate(100);
    dungeon.print();
}
