use crate::dungeon::{dungeon, room};
use crate::lighting::light;


pub fn manage_light(
    lights: &Vec<light::Light>, 
    dungeon: &mut dungeon::Dungeon
) {
    for room in dungeon.get_empty_rooms() {
        dungeon.light_or_unlight_room_at_position(
            is_room_visable(&lights, &room), 
            &room.position
        );
    }
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
