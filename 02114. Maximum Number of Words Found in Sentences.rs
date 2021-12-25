use std::cmp::max;

impl Solution {
    // Loop through the sentences one-by-one and count the words for each sentence. Keep track of the highest count during the looping & return that in the end
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut result = 0;
        for sentence in sentences {
            result = max(result, sentence.split_whitespace().count());
        }
        return result as i32;
    }
}
