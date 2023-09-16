impl Solution {
    // Iterate through the vector and count how many times the next number is lower than the current number. If it happens more than once, the vector can't be sorted with right shifting. If it happens exactly once, check at what index the next number is lower than current. Shifting to right has to happen from that index till the end. Also check if first number is higher than last number for edge cases
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        let mut shifts = 0;
        let size = nums.len();
        for index in 0..(size - 1) {
            if nums[index] > nums[index + 1] {
                counter += 1;
                shifts = (size - (index + 1)) as i32;
            }
        }
        
        // Check if last number is greater than first number since this will become relevant when shifting occurs
        if nums[size - 1] > nums[0] {
            counter += 1;
            // If this was the only place where shifting should have happened, then no shifting is needed, else this is the 2nd place that needs shifting which means vector can't be sorted
            if counter == 1 {
                shifts = 0;
            } else {
                shifts = (size - 1) as i32;
            }
        }
        if counter < 2 {
            return shifts;
        }
        return -1;
    }
}
