use crate::entities::position;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    brightness: i32,
    position: position::Position,
}


impl Light {
    pub fn new(brightness: i32, position: position::Position) -> Light {
        Light { brightness, position }
    }

    pub fn get_brightness(&self) -> i32 {
        self.brightness
    }

    pub fn get_position(&self) -> &position::Position {
        &self.position
    }

    pub fn set_pos(&mut self, position: position::Position) {
        self.position = position;
    }
}
