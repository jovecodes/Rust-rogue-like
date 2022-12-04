use serde_derive::{Serialize, Deserialize};

use crate::entities::position;


#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug, Deserialize, Serialize)]
pub struct Room {
    pub art: char,
    pub position: position::Position,
    pub health: i32,
    pub is_lighted: bool,
}

impl Room {
    pub fn new(art: char, position: position::Position, health: i32) -> Room {
        Room { art , position, health, is_lighted: false }
    }
    
}
