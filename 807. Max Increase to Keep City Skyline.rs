use std::cmp;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let vertical_size: usize = grid.len();
        let horizontal_size: usize = grid[0].len();
        
        // Maintain 2 vectors, one for highest in each row and one for highest in each column
        let mut horizontal_highest = vec![0; horizontal_size];
        let mut vertical_highest = vec![0; vertical_size];

        let mut i: usize = 0;
        let mut j: usize;
        
        let mut result = 0;

        loop {
            if (i == vertical_size) {
                break;
            }
            j = 0;
            loop {
                if (j == horizontal_size) {
                    break;
                }
                
                // Mark as vertical / horizontal highest if it is the highest we've seen till now
                vertical_highest[i] = cmp::max(vertical_highest[i], grid[i][j]);
                horizontal_highest[j] = cmp::max(horizontal_highest[j], grid[i][j]);
                
                j += 1;
            }
            i += 1;
        }
        
        i = 0;
        loop {
            if (i == vertical_size) {
                break;
            }
            j = 0;
            loop {
                if (j == horizontal_size) {
                    break;
                }
                // For every point (i, j), add to result the smallest number out of the 2 highest values in all those 'i's and 'j's
                result += cmp::min(vertical_highest[i], horizontal_highest[j]) - grid[i][j];
                j += 1;
            }
            i += 1;
        }
        // println!("Vertical Skyline: {:?}", horizontal_highest);
        // println!("Horizontal Skyline: {:?}", vertical_highest);
        return result;
    }
}
