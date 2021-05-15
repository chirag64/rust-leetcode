impl Solution {
    // Create new box called result and rotate box_contents to store in result vector by manipulating rows and columns. Then loop through result vector column-wise and reverse-row wise, i.e. bottom to top, column by column. Maintain a counter called last available cell which will be used to place the next '#' and will update every time we see a '*'
    pub fn rotate_the_box(box_contents: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let rows_count = box_contents.len();
        let columns_count = box_contents[0].len();
        let mut result = vec![vec![]; columns_count];
        for i in 0..columns_count {
            result[i] = vec!['0'; rows_count];
        }
        for (row_index, row) in box_contents.iter().enumerate() {
            for (column_index, cell) in row.iter().enumerate() {
                result[column_index][rows_count - row_index - 1] = *cell;
            }
        }
        
        for i in 0..rows_count {
            let mut last_available_cell = columns_count - 1;
            for j in (0..columns_count).rev() {
                if result[j][i] == '*' {
                    last_available_cell = j - 1;
                } else if result[j][i] == '#' {
                    result[j][i] = '.';
                    result[last_available_cell][i] = '#';
                    if last_available_cell != 0 {
                        last_available_cell = last_available_cell - 1;
                    }
                }
            }
        }
        
        return result;
    }
}
