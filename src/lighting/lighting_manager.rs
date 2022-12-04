use std::collections::HashMap;

use crate::entities::position;
use crate::dungeon::{dungeon, room};
use crate::lighting::light;


pub fn manage_light(
    lights: &Vec<light::Light>, 
    dungeon: &dungeon::Dungeon
) -> HashMap<position::Position, room::Room>{
    let mut new_rooms = dungeon.rooms.clone();
    for room in new_rooms.values_mut() {
        room.is_lighted = is_room_visable(&lights, room);
    }
    new_rooms
}


fn is_room_visable(
    lights: &Vec<light::Light>,
    room: &room::Room,
) -> bool {
   for light in lights {
       if room.position.distance_to(light.get_position()) <= light.get_brightness() as f32 {
            return true;
       }
   }
   false
} 
