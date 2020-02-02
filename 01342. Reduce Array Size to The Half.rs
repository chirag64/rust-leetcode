use std::collections::HashMap;


// Count number of instances of each element. Then sort the counts in descending order. Now add them until you reach a sum that is larger than or equal to half of size of arr
impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        
        // Count instances of each element
        let mut map = HashMap::new();
        for element in &arr {
            let count = map.entry(element).or_insert(0);
            *count += 1;
        }
        
        // Sort the counts in descending order
        let mut count_vec: Vec<_> = map.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        
        // Add the count instances until the sum is larger than or equal to half the size of arr
        let mut result = 0;
        let mut current_size = 0;
        let mut target_size = (arr.len() + 1) / 2;
        let mut i = 0;
        loop {
            if (current_size >= target_size || i < 0) {
                break;
            }
            result += 1;
            current_size += count_vec[i].1;
            i += 1;
            
        }
        return result;
    }
}
