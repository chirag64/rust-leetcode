impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        // Calculate in each step where the robot goes. If at the end, robot is on the same spot as the starting point or robot ends by facing a direction different than up, then it is going in circles
        
        // Directions: 0 - up, 1 - right, 2 - down, 3 - left
        let mut current_direction: i32 = 0;
        let mut current_position = [0, 0].to_vec();
        let size = instructions.len();
        let mut i = 0;
        loop {
            if (i == size) {
                return (current_direction != 0) || ((current_position[0] == 0) && (current_position[1] == 0));
            }
            let current_character = instructions.chars().nth(i).unwrap();
            match current_character {
                // Adding 4 so that direction does not go in negative
                'R' => current_direction = (current_direction + 4 + 1) % 4,
                'L' => current_direction = (current_direction + 4 - 1) % 4,
                'G' => {
                        match current_direction {
                            0 => current_position[1] += 1,
                            1 => current_position[0] += 1,
                            2 => current_position[1] -= 1,
                            3 =>  current_position[0] -= 1,
                            _ => println!("Current direction: {}", current_direction)
                        }
                    },
                _ => println!("Current character: {}", current_character)
            }
            // println!("Current position: {:?}, Current Direction: {:?}, Current Character: {:?}", current_position, current_direction, current_character);
            i += 1;
        }
    }
}
