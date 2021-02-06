use std::collections::HashMap;

impl Solution {
    // Keep count of instances of each number inside the vector. If it is counted first time, add it to the sum. If it is counted a second time, subtract it back from the sum since it need not be counted and it was added earlier. For all subsequent times a number is counted, keep counting it but ignore it from the sum
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        
        // Count instances of each element
        let mut map = HashMap::new();
        for num in &nums {
            let count = map.entry(num).or_insert(0);
            *count += 1;
            
            if *count == 1 {
                sum += num;
            }
            
            if *count == 2 {
                sum -= num;
            }
        }
        
        return sum;
    }
}
