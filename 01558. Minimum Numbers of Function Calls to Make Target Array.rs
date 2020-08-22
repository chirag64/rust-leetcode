impl Solution {
    // Do the reverse operation, i.e. convert input vector into a vector of zeroes. Loop through vector and if you find an odd number, subtract 1. Every subtraction counts as a step. Then divide the entire vector by 2. This counts as 1 step. Repeat these two steps until entire vector
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut output = 0;
        let mut i = 0;
        let size = nums.len();
        let mut all_numbers_are_zero = false;
        while !all_numbers_are_zero {
            i = 0;
            all_numbers_are_zero = true;
            while i != size {
                if nums[i] % 2 == 1 {
                    nums[i] -= 1;
                    output += 1;
                }
                
                if nums[i] != 0 {
                    all_numbers_are_zero = false;
                }                
                i += 1;
            }
                
            if !all_numbers_are_zero {
                i = 0;
                while i != size {
                    nums[i] /= 2;
                    i += 1;
                }
                output += 1;
            }
        }
        return output;
    }
}
