use std::cmp::max;

impl Solution {
    // Keep track of highest num and sum of score instead of calculating score vector everytime. For every iteration of nums, add previous score to highest_num and current num
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        let mut ans:Vec<i64> = Vec::new();
        let mut highest_num = 0;
        let mut score = 0;
        
        for num in nums {
            highest_num = max(highest_num, num as i64);
            score += (highest_num + num as i64);
            ans.push(score);
        }
        
        return ans;
    }
}
