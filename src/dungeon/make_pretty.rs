fn make_pretty(dungeon: Dungeon) {
    to_do();


    pub fn generate(&mut self) {
        let mut room_to_generate = self.fully_generated();
        
        while room_to_generate != 0 {
            self.generate_room(room_to_generate);
            room_to_generate = self.fully_generated();
        }
        
    }

    fn generate_room(&mut self, index: usize) {
        let size_x : usize = self.size_x.try_into().unwrap();
        
        let left = self.rooms[index - 1].right;

        let right = self.rooms[index + 1].left;
        let up = self.rooms[index - size_x].down;
        let down = self.rooms[index + size_x].up;

        self.rooms[index] = Dungeon::get_fitting_room(left, right, down, up);
        self.print();
    }


    fn fully_generated(&mut self) -> usize {
        for i in 0..(self.size_x * self.size_y) {
            let usize_i : usize = i.try_into().unwrap(); 
            if self.rooms[usize_i].art != EMPTY_ROOM.art {

                let room_to_generate = self.check_tile(usize_i);
                if room_to_generate != 0 {
                    return room_to_generate;
                }
            } 
        }
       return 0; 

    }


    fn check_tile(&self, index: usize) -> usize {
        let mut compleated = 0;
        let size_x : usize = self.size_x.try_into().unwrap();
        
        // Left
        if self.rooms[index - 1].right == false && self.rooms[index].left == true {
            compleated = index - 1;
            return compleated;
        }

        // Right
        if self.rooms[index + 1].left == false && self.rooms[index].right == true {
            compleated = index + 1;
            return compleated;
        }

        // Down
        if self.rooms[index + size_x].up == false && self.rooms[index].down == true {
            compleated = index + size_x;
            return compleated;
        }

        // Up
        if self.rooms[index - size_x].down == false && self.rooms[index].up == true {
            compleated = index - size_x;
            return compleated;
        }
        return compleated;

    }


    fn get_fitting_room(left: bool, right: bool, down: bool, up: bool) -> Room {
        let mut  rooms = vec![
            EMPTY_ROOM, 
            OPEN_ROOM, 
            HORIZONTAL_ROOM, 
            VERTICLE_ROOM, 
            RIGHT_ROOM, 
            LEFT_ROOM, 
            DOWN_ROOM, 
            UP_ROOM
        ];
            
        if left {
            rooms.retain(|&x| x.left == true);
        } else {
            rooms.retain(|&x| x.left != true); 
        }

        if right {
            rooms.retain(|&x| x.right == true);
        } else { 
            rooms.retain(|&x| x.right != true); 
        }

        if down {
            rooms.retain(|&x| x.down == true);
        } else {
            rooms.retain(|&x| x.down != true); 
        }

        if up {
            rooms.retain(|&x| x.up == true);
        } else {
            rooms.retain(|&x| x.up != true); 
        }
        
        if rooms.len() <= 0 {
            return EMPTY_ROOM;
        }
        println!("{:?}",rooms);
        return rooms[rand::thread_rng().gen_range(0..rooms.len())];
    }
}
