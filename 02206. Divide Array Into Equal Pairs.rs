use std::collections::HashMap;

impl Solution {
    // Maintain a hashmap to keep count of each number's occurrence. If count of any number is odd, return false. In the end, return true since that means no number occurred odd number of times
    pub fn divide_array(nums: Vec<i32>) -> bool {

        let mut map = HashMap::new();
        for num in &nums {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }
        
        for (num, num_instances) in &map {
            if num_instances % 2 != 0 {
                return false;
            }
        }
        
        return true;
    }
}
