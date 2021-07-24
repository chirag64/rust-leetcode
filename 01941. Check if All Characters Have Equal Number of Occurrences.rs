use std::collections::HashMap;

impl Solution {
    // Count occurrences of every character. Then compare them with a counter that you maintained in first loop. If any of them don't match the counter, return false, else return true
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut number_of_occurrences = 0;
        let mut map = HashMap::new();
        
        for ch in s.chars() {
            let count = map.entry(ch).or_insert(0);
            *count += 1;
            number_of_occurrences = *count;
        }
        
        for (ch, occurrences) in &map {
            if *occurrences != number_of_occurrences {
                return false;
            }
        }
        
        return true;
    }
}
