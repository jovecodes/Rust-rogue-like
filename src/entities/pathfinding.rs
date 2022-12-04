use crate::entities::position::Position;


pub fn pathfind(
    current: &Position, 
    target: &Position, 
) -> Position {
    let mut point_vector = Position::new(target.x - current.x, target.y - current.y);
    point_vector.become_direction();

    return point_vector;
}

