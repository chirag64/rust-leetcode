impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut old_grid = grid.to_vec();
        let mut new_grid = grid.to_vec();

        // Do the entire operation k times
        for x in 0..k {
            // Shift entire column by 1
            for y in 0..rows {
                for z in 0..cols {
                    new_grid[y][(z + 1) % cols] = old_grid[y][z];
                }
            }
            
            // Shift entire first row by -1
            for y in 0..rows {
                new_grid[y][0] = old_grid[(y + rows - 1) % rows][cols - 1];
            }
            
            // Update old grid to new grid
            old_grid = new_grid.to_vec();
        }
        return new_grid;
    }
}
