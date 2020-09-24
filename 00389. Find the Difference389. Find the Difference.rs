use std::collections::HashMap;

impl Solution {
    // Count instances of each character in s in a hashmap. Then subtract those instances from the same hashmap while iterating through t. If any character does not exist in hashmap or is left 0 times while iterating t, it means that is the extra character added in t
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut map = HashMap::new();
        for ch in s.chars() {
            let count = map.entry(ch).or_insert(0);
            *count += 1;
        }
        for ch in t.chars() {
            let count = map.entry(ch).or_insert(0);
            if *count == 0 {
                return ch;
            }
            *count -= 1;
        }
        return '0';
    }
}
