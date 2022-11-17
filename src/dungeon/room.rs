use crate::dungeon::position;


#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Room {
    pub art: char,
    pub position: position::Position
}

impl Room {
    pub fn new(art: char, position: position::Position) -> Room {
        Room { art , position }
    }
    
}
