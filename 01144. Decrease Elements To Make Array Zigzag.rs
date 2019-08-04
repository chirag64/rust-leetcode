use std::cmp;

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut evenCount = 0;
        let mut oddCount = 0;
        let mut i = 0;
        let count = nums.len();
        
        // Keep count of both even and odd level instances and finally return the minimum of the two
        loop {
            if (i == count) {
                break;
            }
            if (i == 0) {
                println!("Found 0: {}", nums[i] - nums[i + 1] + 1);
                // 0
                oddCount += cmp::max(0, nums[i] - nums[i + 1] + 1);
            } else if (i % 2 == 0) {
                if (i + 1 == count) {
                    println!("Found odd and last: {}", nums[i] - nums[i - 1] + 1);
                    oddCount += cmp::max(0, nums[i] - nums[i - 1] + 1);
                } else {
                    println!("Found odd: {}", nums[i] - cmp::min(nums[i - 1], nums[i + 1]) + 1);
                    // Odd
                    oddCount += cmp::max(0, nums[i] - cmp::min(nums[i - 1], nums[i + 1]) + 1);
                }
            } else {
                if (i + 1 == count) {
                    println!("Found even and last: {}", nums[i] - nums[i - 1] + 1);
                    evenCount += cmp::max(0, nums[i] - nums[i - 1] + 1);
                } else {
                    // Even
                    println!("Found even: {}", nums[i] - cmp::min(nums[i - 1], nums[i + 1]) + 1);
                    evenCount += cmp::max(0, nums[i] - cmp::min(nums[i - 1], nums[i + 1]) + 1);
                }
            }
            i += 1;
        }
        println!("evenCount: {}, oddCount: {}", evenCount, oddCount);
        return cmp::min(evenCount, oddCount)
    }
}
