impl Solution {
    // Calculate the total of all nums (remember to use i64 for storing total as i32 range won't be enough). This is now right total. Then loop through nums, subtracting each number from right total and adding it to left total. Count the number of times left total is higher than or equal to right total while looping and then return that count.
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut left_total: i64 = 0;
        let mut right_total: i64 = 0;
        let len = nums.len();
        let mut valid_splits = 0;
        
        for num in &nums {
            right_total += *num as i64;
        }
        
        for num in &nums[0..(len - 1)] {
            left_total += *num as i64;
            right_total -= *num as i64;
            if left_total >= right_total {
                valid_splits += 1;
            }
        }
        
        return valid_splits;
    }
}
