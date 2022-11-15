pub const EMPTY_ROOM: Room = Room{art: '#'};
pub const OPEN_ROOM:  Room = Room{art: '.'};
pub const GOAL_ROOM:  Room = Room{art: 'G'};

//const HORIZONTAL_ROOM: Room = Room{art: '=', left: true,  right: true,  down: false, up: false};
//const VERTICLE_ROOM:   Room = Room{art: '|', left: false, right: false, down: true,  up: true };
//
//const RIGHT_ROOM:      Room = Room{art: '[', left: false, right: true,  down: false, up: false};
//const LEFT_ROOM:       Room = Room{art: ']', left: true,  right: false, down: false, up: false};
//const DOWN_ROOM:       Room = Room{art: 'M', left: false, right: false, down: true,  up: false};
//const UP_ROOM:         Room = Room{art: 'U', left: false, right: false, down: false, up: true };


#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct Room {
    pub art: char,
}
