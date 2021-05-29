use std::cmp::max;

impl Solution {
    // Sort the vector and then create pairs by picking numbers from both ends to make the smallest of sums. Keep track of the biggest sum by looping through these
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut max_sum = 0;
        
        nums.sort();
        
        for i in 0..(len / 2) {
            max_sum = max(max_sum, nums[i] + nums[len - i - 1]);
        }
        return max_sum;
    }
}
