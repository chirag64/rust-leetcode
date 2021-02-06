use std::cmp;

impl Solution {
    // Keep calculating sum and keep tracking when the sum was minimum and maximum throughout the loop. Result is the difference between the max sum and min sum
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut min = 0;
        let mut max = 0;
        let mut current_sum = 0;
        
        for num in nums {
            current_sum += num;
            min = cmp::min(current_sum, min);
            max = cmp::max(current_sum, max);
        }
        
        return max - min;
    }
}
