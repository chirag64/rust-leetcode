use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut map = HashMap::new();
        let mut answer = 0;
        
        // Count characters in chars and store them in a hashmap
        for ch in chars.chars() {
            let counter = map.entry(ch).or_insert(0);
            *counter += 1;
        }
        
        // Create copy of above calculated hashmap, then reduce the characters from that hashmap as per the letters in the word for each word. If we ever reach 0, the word cannot be made using those characters
        for word in words {
            let mut new_map = map.clone();
            for (index, ch) in word.chars().enumerate() {
                let counter = new_map.entry(ch).or_default();
                if (*counter == 0) {
                    break;
                } else {
                    *counter -= 1;
                }
                if (index + 1 == word.len()) {
                    answer += word.len() as i32;
                }
            }
        }
        return answer;
    }    
}
