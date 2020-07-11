impl Solution {
    // Run a loop of 3 levels to iterate through the vector as per the instructions and create a new vector which holds sum of subvectors. Then sort this new vector & iterate through it from left to right positions and hold the sum
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut vec = Vec::new();
        let mut result = 0;
        for (outer_index, i) in nums.iter().enumerate() {
            for j in outer_index..(n as usize) {
                let mut continous_subarray_sum = 0;
                for k in outer_index..(j + 1) {
                    continous_subarray_sum += nums[k];
                }                
                vec.push(continous_subarray_sum);
            }
        }
        vec.sort();
        for i in left..(right + 1) {
            result += vec[(i - 1) as usize];
        }
        return result;
    }
}
