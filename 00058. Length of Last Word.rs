impl Solution {
    // Split string by white space into an vector of strings. Return length of last string in vector
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() > 0 {
            return words[words.len() - 1].len() as i32;
        }
        return 0;
    }
}
