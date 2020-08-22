use std::collections::HashSet;

// Run a recursive function for every character that checks if a cycle exists for that character. The function will keep a track of which direction it came from and will try to go in any other direction. If it finds the same character in any of those directions and it knows it has not exploerd that place before (using a HashSet to keep track of those places), go in that direction and repeat process. If we ever find the same character in a direction & position where we have been earlier, return out of all functions and simply return true
impl Solution {
    
    // Recursive function to check if a cycle exists of the current character parameter. The main function calls this function once for position that it has not covered in the past
    pub fn is_it_a_cycle(grid: &Vec<Vec<char>>, i: usize, j: usize, ch: char, positions_explored: &mut HashSet<usize>, last_direction: &str) -> bool {
        positions_explored.insert((i * grid[0].len()) + j);
        // println!("{}: {} {} - {}", ch, i, j, last_direction);
        // println!("{:?}", positions_explored);
        if last_direction != "RIGHT" && (j != 0) && ch == grid[i][j - 1] && positions_explored.contains(&(i * grid[0].len() + (j - 1))) {
            return true;
        }
        if last_direction != "UP" && (i != grid.len() - 1) && ch == grid[i + 1][j] && positions_explored.contains(&((i + 1) * grid[0].len() + j)) {
            return true;
        }
        if last_direction != "LEFT" && (j != (grid[0].len() - 1)) && ch == grid[i][j + 1] && positions_explored.contains(&(i * grid[0].len() + (j + 1))) {
            return true;
        }
        if last_direction != "DOWN" && (i != 0) && ch == grid[i - 1][j] && positions_explored.contains(&((i - 1) * grid[0].len() + j)) {
            return true;
        }
        if last_direction != "RIGHT" && (j != 0) && ch == grid[i][j - 1] {
            let cycle_exists = Solution::is_it_a_cycle(&grid, i, j - 1, ch, positions_explored, "LEFT");
            if cycle_exists {return cycle_exists;}
        }
        if last_direction != "UP" && (i != grid.len() - 1) && ch == grid[i + 1][j] {
            let cycle_exists = Solution::is_it_a_cycle(&grid, i + 1, j, ch, positions_explored, "DOWN");
            if cycle_exists {return cycle_exists;}
        }
        if last_direction != "LEFT" && (j != grid[0].len() - 1) && ch == grid[i][j + 1] {
            let cycle_exists = Solution::is_it_a_cycle(&grid, i, j + 1, ch, positions_explored, "RIGHT");
            if cycle_exists {return cycle_exists;}
        }
        if last_direction != "DOWN" && (i != 0) && ch == grid[i - 1][j] {
            let cycle_exists = Solution::is_it_a_cycle(&grid, i - 1, j, ch, positions_explored, "UP");
            if cycle_exists {return cycle_exists;}
        }    
        return false;
    }
    
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let mut positions_explored = HashSet::new();
        let mut contains_cycle = false;
        for (outer_index, i) in grid.iter().enumerate() {
            for (inner_index, j) in i.iter().enumerate() {
                let current_char = j;
                
                if !positions_explored.contains(&((outer_index * grid[0].len()) + inner_index)) {
                    contains_cycle = Solution::is_it_a_cycle(&grid, outer_index, inner_index, *current_char, &mut positions_explored, "");
                    if (contains_cycle) {
                        return true;
                    }
                }
            }
        }
        return contains_cycle;
    }
}
