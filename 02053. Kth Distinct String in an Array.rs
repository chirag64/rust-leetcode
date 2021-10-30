use std::collections::HashMap;

impl Solution {
    // Keep adding strings from arr within a HashMap named 'map' and maintain count of each string's instances. Then loop through arr again and count which ones have a count value of 1 in the HashMap. When you reach the kth string in the arr which has a count of 1 in HashMap, return it
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut map = HashMap::new();
        let mut i = 0;
        
        // Maintain count of instances of each string
        for st in &arr {
            let count = map.entry(st.to_string()).or_insert(0);
            *count += 1;
        }
        
        // Find the kth string that has a count of 1 in map
        for st in &arr {
            if *(map.get_mut(st).unwrap()) == 1 {
                i += 1;
                if i == k {
                    return st.to_string();
                }
            }
        }
        
        return "".to_string();
    }
}
