#[macro_use] extern crate magic_crypt;
mod dungeon;
mod lighting;
mod entities;
mod saver_loader;

fn main() {
    dungeon::play();
}

/*
TODO Plans:

1. vision / fog of war
2. lighting
3. only spawning in light

4. enemy pathfinding
5. enemy mining

6. Ore generation
*/ 
