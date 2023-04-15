use std::cmp::max;

impl Solution {
    // Run nested loop on grid. If we are checking 0th number in inner vector, add it to result vector. Else compare the previous highest (length) with current (length).
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        
        for (i, row) in grid.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                let mut length = num.to_string().len();
                if i == 0 {
                    result.push(length as i32);
                } else {
                    result[j] = max(result[j], length as i32);
                }
            }
        }
        
        return result;
    }
}
