use std::collections::HashMap;

// Run a nested for loop and check every number with every other number whether the other number is twice of the number
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        for (outer_index, outer_val) in arr.iter().enumerate() {
            for (inner_index, inner_val) in arr.iter().enumerate() {
                if (outer_index != inner_index && *outer_val * 2 == *inner_val) {
                    return true;
                }
            }
        }
        return false;
    }
}
