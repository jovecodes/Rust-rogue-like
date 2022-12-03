use serde_derive::{Serialize, Deserialize};

use crate::dungeon::position;


#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug, Deserialize, Serialize)]
pub struct Room {
    pub art: char,
    pub position: position::Position,
    pub health: i32,
}

impl Room {
    pub fn new(art: char, position: position::Position, health: i32) -> Room {
        Room { art , position, health }
    }
    
}
