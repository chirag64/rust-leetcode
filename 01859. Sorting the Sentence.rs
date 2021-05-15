impl Solution {
    // Split sentence into a vector. Then create a blank vector of same size. Then loop through the first vector and add words in the second vector as per the number within the string. Then join the second vector and return it
    pub fn sort_sentence(s: String) -> String {
        let words_split: Vec<_> = s.split(" ").collect();
        
        let mut result_words_split = vec![""; words_split.len()];
        
        for word in words_split {
            let position_index = (word.chars().nth(word.chars().count() - 1).unwrap()) as usize - 49;
            result_words_split[position_index] = &word[0..(word.chars().count() - 1) as usize];
        }
        
        return result_words_split.join(" ");
    }
}
