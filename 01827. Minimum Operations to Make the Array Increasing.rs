impl Solution {
    // Loop from 0 to n and make sure every (i + 1)th number is at least 1 more than ith number. If it isn't, count the number to add to make it so and add it in result.
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut current = nums[0];
        let mut result = 0;
        for i in 0..(len - 1) {
            if nums[i + 1] <= current {
                result += ((current + 1) - nums[i + 1]);
                current += 1;
            } else {
                current = nums[i + 1];
            }
        }
        return result;
    }
}
