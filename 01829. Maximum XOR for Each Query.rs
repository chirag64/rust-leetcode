impl Solution {
    // Save first number as xor_result. Save it as last_xor and then calculate XOR of xor_result with (pow(2, max_bit) - 1). Push this result in the result vector and then reinitialize xor_result with last_xor. Then perform XOR between xor_result and second number...
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let len = nums.len();
        let mut i = 1;
        let mut result = Vec::new();
        let mut xor_result = nums[0];
        let mut last_xor = xor_result;
        while i <= len {
            // Re-initialize xor_result from last iteration where xor_result is xor of first i numbers
            xor_result = last_xor;
            
            // Don't calculate XOR if we are still on first number
            if i != 1 {
                xor_result = xor_result ^ nums[i - 1];
            }
            
            // Save xor_result for next iteration
            last_xor = xor_result;
            xor_result = xor_result ^ (i32::pow(2, maximum_bit as u32) - 1);
            result.insert(0, xor_result);
            i += 1;
        }
        return result;
    }
}
