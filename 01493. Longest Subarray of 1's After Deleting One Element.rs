impl Solution {
    // Maintain 2 counters. Add 1 in both during each iteration of loop of nums. If you get a 0, check the highest of 2 and assign it to max_count (but reduce 1 because it has counted a 0 in it which needs to be removed). Then make counter2 what was counter1 and reset counter1. Basically counter1 is counting from last 0 to current and counter2 is counting from 2nd last 0 to current.
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut counter1 = 0;
        let mut counter2 = 0;
        let mut max_count = 0;
        let mut i = 0;
        loop {
            if i == nums.len() {
                max_count = std::cmp::max(max_count, counter1);
                max_count = std::cmp::max(max_count, counter2);
                return max_count - 1;
            }
            counter1 += 1;
            counter2 += 1;
            if nums[i] == 0 {
                max_count = std::cmp::max(max_count, counter1 - 1);
                max_count = std::cmp::max(max_count, counter2 - 1);
                counter2 = counter1;
                counter1 = 0;
            }
            i += 1;
        }
    }
}
