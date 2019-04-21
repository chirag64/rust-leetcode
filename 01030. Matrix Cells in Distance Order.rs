impl Solution {
    
    // Brute force approach - We go from 0 to r, 0 to c and do insertion sort with distance
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut ordered_cells = Vec::new();
        //println!("{}, {}, {}, {}", r, c, r0, c0);
        for i in 0..r {
            for j in 0..c {
                //ordered_cells.push([i, j].to_vec());
                ordered_cells = Solution::insertion_sort(ordered_cells, [i, j].to_vec(), r0, c0);
            }
        }
        //println!("{:?}", ordered_cells);
        return ordered_cells;
    }
    
    // Insertion sort of new cell in existing ordered_cells vector
    fn insertion_sort(mut ordered_cells: Vec<Vec<i32>>, new_cell: Vec<i32>, r0: i32, c0: i32) -> Vec<Vec<i32>>{
        let mut i = ordered_cells.len();
        ordered_cells.push(new_cell);
        loop {
            //println!("inner i is {}", i);
            if (i == 0) {
                break;
            }
            let current_cell_distance = (ordered_cells[i][0] - r0).abs() + (ordered_cells[i][1] - c0).abs();
            let previous_cell_distance = (ordered_cells[i - 1][0] - r0).abs() + (ordered_cells[i - 1][1] - c0).abs();
            
            // If distance of i-th cell is less than that of previous cell, swap them since we need the ordered_cells vector in the ascending order of vectors from distance of r0, c0
            if (current_cell_distance < previous_cell_distance) {
                ordered_cells.swap(i, i - 1);
            } else {
                break;
            }
            i -= 1;
        }
        return ordered_cells;
    }    
}
