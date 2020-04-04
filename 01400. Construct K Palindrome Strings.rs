use std::collections::HashMap;

// Count instances of each letter. Count how many have an odd count. You need to have at least odd_count number of individual letter strings and the rest of the even numbered letters can form one or two letter strings as needed. Also, there should at least be 'k' number of letters in 's' for you to be able to create 'k' strings
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut map = HashMap::new();
        let size = s.len() as i32;
        
        let mut odds = 0;

        for char in s.chars() {
            let count = map.entry(char).or_insert(0);
            *count += 1;
        }
        
        for (key, value) in &map {
            if value % 2 != 0 {
                odds += 1;
            }
        }
        
        return (size >= k && odds <= k);
    }
}
