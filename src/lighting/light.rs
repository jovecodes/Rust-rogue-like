use dungeon::position;

pub struct Light {
    brightness: i32,
    position: position::Position,
}


impl Light {
    pub fn get_brightness(&self) -> i32 {
        self.brightness
    }

    pub get_position(&self) -> &position::Position {
        &self.position
    }
}
