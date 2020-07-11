use std::cmp::min;

impl Solution {
    // Sort nums vector. Calculate the differences between ith element from left and (3 - i)th element from right. Keep track of lowest difference and return it
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        
        let mut sorted_nums = nums.to_vec();
        let size = nums.len();
        let mut smallest_difference = std::i32::MAX;
        let mut i = 0;
        
        if size < 4 {
            return 0;
        }
        
        sorted_nums.sort();
        
        while i < 4 {
            smallest_difference = min(smallest_difference, sorted_nums[size - (4 - i)] - sorted_nums[i]);
            i += 1;
        }
        
        return smallest_difference;
    }
}
