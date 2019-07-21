use std::cmp;

impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut max = 0;
        let size = arr1.len();
        let mut i = 0;
        let mut j = 0;
        loop {
            if (i == size) {
                break;
            }
            j = i;
            loop {
                if (j == size) {
                    break;
                }
                // Simply calculate using normal brute-force approach
                max = cmp::max(max, ((arr1[i] - arr1[j]).abs() + (arr2[i] - arr2[j]).abs() + (i as i32 - j as i32).abs()));
                j += 1;
            }
            i += 1;
        }
        return max;
    }
}
