use std::collections::HashMap;

impl Solution {
    // Create 2 HashMaps maintaining count of each character in each word. Then loop through one of the hashmaps and check the difference in counts of that character in both hashmap. If the difference is ever greater than 3, return false, else return true in end. While looping through the both words, add their entries in the other word's hashmap so that both word's hashmaps have a track of all characters used in word1 & word2. This will ensure that while looping through one of the word's hashmap in the end, we will cover every character that exists in both words
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut word1_hm = HashMap::new();
        let mut word2_hm = HashMap::new();
        
        for c in word1.chars() {
            let count = word1_hm.entry(c).or_insert(0);
            *count += 1;
            
            word2_hm.entry(c).or_insert(0);
        }
        
        for c in word2.chars() {
            let count = word2_hm.entry(c).or_insert(0);
            *count += 1;
            
            word1_hm.entry(c).or_insert(0);
        }
        
        for (key, value) in &word1_hm {
            if ((*(&word1_hm.get(key).unwrap()) - *(&word2_hm.get(key).unwrap())) as i32).abs() > 3 {
                return false;
            }            
        }
        
        return true;
    }
}
