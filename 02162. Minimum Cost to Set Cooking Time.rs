use std::cmp::min;

impl Solution {
    // Create a direct solution with mins:secs and indirect solution with (mins - 1):(secs + 60) and calculate the cost for executing both. Return the minimum cost
    pub fn min_cost_set_time(start_at: i32, move_cost: i32, push_cost: i32, target_seconds: i32) -> i32 {
        let mut direct_mins = (target_seconds / 60).to_string();
        let mut direct_seconds = (target_seconds % 60).to_string();
        let mut indirect_mins = ((target_seconds / 60) - 1).to_string();
        let mut indirect_seconds = ((target_seconds % 60) + 60).to_string();
        // If direct mins are 0, indirect mins should be 0 (not -1) and don't add 60 to indirect secs
        if (target_seconds / 60) == 0 {
            indirect_mins = "0".to_owned();
            indirect_seconds = direct_seconds.to_string();            
        }
        // If direct secs are less than 10, preprend a 0
        if (target_seconds % 60) < 10 {
            direct_seconds = "0".to_owned() + &direct_seconds.to_string();
        }
        
        // Generate direct and indirect solutions as strings
        let direct_solution = direct_mins.to_string() + &direct_seconds.to_string();
        let indirect_solution = indirect_mins.to_string() + &indirect_seconds.to_string();
        // println!("Direct Solution: {}, Indirect Solution: {}", direct_solution, indirect_solution);
        let mut current_cost = i32::MAX;
        let mut min_cost = i32::MAX;
        let mut last_position = char::from_digit(start_at as u32, 10).unwrap();
        // Only execute if direct minutes is less than 100 since we can only enter max value of 99
        if (target_seconds / 60) < 100 {
            current_cost = 0;
            for current_position in direct_solution.chars().collect::<Vec<char>>() {
                // Don't start adding cost if nothing has been typed so far and the number to be typed is 0
                if !(current_cost == 0 && current_position == '0') {
                    if last_position != current_position {
                        // println!("Last Position: {}, Current Position: {}", last_position, current_position);
                        current_cost += move_cost;
                        last_position = current_position;
                    }
                    if !(current_cost == 0 && current_position == '0') {
                        current_cost += push_cost;
                    }
                }
            }
            min_cost = min(min_cost, current_cost);
        }
        // Only calculate this if indirect seconds isn't more than 100, because that can't be entered in microwave
        if (target_seconds % 60) < 50 {
            // Reset values for current cost and last position
            current_cost = 0;
            last_position = char::from_digit(start_at as u32, 10).unwrap();
            for current_position in indirect_solution.chars().collect::<Vec<char>>() {
                // Don't start adding cost if nothing has been typed so far and the number to be typed is 0
                if !(current_cost == 0 && current_position == '0') {
                    if last_position != current_position {
                        // println!("Last Position: {}, Current Position: {}", last_position, current_position);
                        current_cost += move_cost;
                        last_position = current_position;
                    }
                    current_cost += push_cost;
                }
            }
            // println!("Current cost: {}", current_cost);
            min_cost = min(min_cost, current_cost);
        }
        return min_cost;
    }
}
