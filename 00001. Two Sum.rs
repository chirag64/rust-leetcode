impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create output vector
        let mut vec: Vec<i32> = Vec::new();
        
        // Outer for loop for first iteration
        for i in 0..nums.len() {
            
            // Inner for loop for second iteration from i to n
            for j in i+1..nums.len() {
                
                // If we get the target sum, immediately get out of the function
                if (nums[i] + nums[j] == target) {
                    vec.push(i as i32);
                    vec.push(j as i32);
                    return vec;                 
                }
            }
        }
        return vec;
    }
}
