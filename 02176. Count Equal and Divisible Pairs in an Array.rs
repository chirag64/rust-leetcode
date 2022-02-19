impl Solution {
    // Create loop within a loop that checks if nums values at both positions are correct and that their product is divisible by k. Return the result at the end    
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let len = nums.len();
        let mut result = 0;
        
        while i < len {
            for j in (i + 1)..len {
                if (nums[i] == nums[j]) && ((i * j) % (k as usize) == 0) {
                    result += 1;
                }
            }
            i += 1;
        }
        return result;
    }
}
