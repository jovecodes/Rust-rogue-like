use crate::entities::position::Position;


pub fn pathfind(
    current: &Position, 
    target: &Position, 
) -> Position {
    let mut point_vector = target.get_minus(current);
    point_vector.become_direction();

    return point_vector;
}

