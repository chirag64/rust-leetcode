impl Solution {
    // Simple brute-force approach. Generate an output grid of same size. Maintain 4 vectors: onesRow, zerosRow, onesCol, zerosCol to maintain counts of 1s and 0s for each row and column. Then iterate through output grid and update the scores by calculating based on values from the 4 vectors
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut rowsCount = grid.len();
        let mut colsCount = grid[0].len();
        let mut output = Vec::new();
        for i in 0..rowsCount {
            output.push(Vec::new());
            for j in 0..colsCount {
                output[i].push(0);
            }
        }
        
        let mut onesRow = vec![0; rowsCount];
        let mut zerosRow = vec![0; rowsCount];
        let mut onesCol = vec![0; colsCount];
        let mut zerosCol = vec![0; colsCount];
        
        for (row_num, row) in grid.iter().enumerate() {
            for (col_num, cell) in row.iter().enumerate() {
                if *cell == 1 {
                    onesRow[row_num] += 1;
                    onesCol[col_num] += 1;
                } else {
                    zerosRow[row_num] += 1;
                    zerosCol[col_num] += 1;
                }
            }            
        }
        
        for i in 0..rowsCount {
            for j in 0..colsCount {
                output[i][j] = onesRow[i] + onesCol[j] - zerosRow[i] - zerosCol[j];
            }
        }
        
        return output;
    }
}
