impl Solution {
    // Create a string called needle. Keep checking if needle exists in sequence. If it does, concatenate word in needle & repeat. Once the substring no longer exists, return the count
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut needle: String = word.clone();
        let mut result = 0;
        while sequence.contains(&needle) {
            result += 1;
            needle.push_str(&word);
        }
        return result;
    }
}
