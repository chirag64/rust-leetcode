use std::collections::HashMap;
use std::cmp;

// Create hashmap for both strings that count the instances of each character. Then loop through s-string's hashmap and count how many of them are not there in t-string
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut s_map = HashMap::new();
        let mut t_map = HashMap::new();
        let mut difference = 0;
        
        // Count instances of each character in s-string
        for char in s.chars() {
            let count = s_map.entry(char).or_insert(0);
            *count += 1;
        }
        
        // Count instances of each character in t-string
        for char in t.chars() {
            let count = t_map.entry(char).or_insert(0);
            *count += 1;
        }

        // Loop through s-string's hashmap and count how many of them are not there in t-string
        for (key, value) in &s_map {
            let s_count = value;
            let mut t_count = 0;
            if t_map.contains_key(key) {
                t_count = *t_map.get_mut(key).unwrap();
            }
            
            // If difference is negative, consider 0
            let current_difference = cmp::max((s_count - t_count) as i32, 0);
            difference += current_difference;
        }
        
        return difference;
    }
}
