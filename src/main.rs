mod dungeon;

fn main() {
    make_dungeon();
}

fn make_dungeon() {
    let mut dungeon = dungeon::walker::Walker::new();
    dungeon.generate(300);
    dungeon.print();
}
