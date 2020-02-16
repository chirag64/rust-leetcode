// Loop through the grids and count the numbers less than 0
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for m in grid {
            for n in m {
                if n < 0 {
                    count += 1;
                }
            }
        }
        return count;
    }
}
