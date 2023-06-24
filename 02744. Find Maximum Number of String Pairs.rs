impl Solution {
    // Create nested loop to loop through words and compare every word with the reverse of every other word after it. Keep count of matches and return matches in the end
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let size = words.len();
        let mut result = 0;
        
        for i in 0..(size - 1) {
            for j in (i + 1)..size {
                if words[i] == words[j].chars().rev().collect::<String>() {
                    result += 1;
                }
            }
        }        
        return result;
    }
}
