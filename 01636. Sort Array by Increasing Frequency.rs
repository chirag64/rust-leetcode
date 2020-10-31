use std::collections::HashMap;

impl Solution {
    
    // Count instances of each num. Then sort those counts in descending order of num value and ascending order of count. Using this sorted counts, generate the result vector
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        
        // Count instances of each element
        let mut map = HashMap::new();
        let mut result = Vec::new();
        for element in &nums {
            let count = map.entry(element).or_insert(0);
            *count += 1;
        }
        
        let mut count_vec: Vec<_> = map.iter().collect();
        
        // Sort the hashmap by key in descending order
        count_vec.sort_by(|a, b| b.0.cmp(a.0));
        
        // Sort the hashmap by value in ascending order
        count_vec.sort_by(|a, b| a.1.cmp(b.1));

        // println!("{:?}", count_vec);
        
        for i in &count_vec {
            for j in 0..*(i.1) {
                result.push(**(i.0));
            }
        }
        return result;
    }
}
