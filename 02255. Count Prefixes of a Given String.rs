use std::cmp::min;

impl Solution {
    // Loop through the words and compare word with substring of same size. If they match, add one to the counter. In the end, return the counter value
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut counter = 0;
        for word in words {
            let smallest_length = min(word.len(), s.len());
            if &s[0..smallest_length] == word {
                counter += 1;
            }
        }
        return counter;
    }
}
