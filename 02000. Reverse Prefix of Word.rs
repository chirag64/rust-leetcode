impl Solution {
    // If char doesn't exist in word, return original word. Else, run a reverse loop from start to char and add those chars in a new string. Then continue new loop from that char to end and add those chars in correct order in same string. Return the newly formed string
    pub fn reverse_prefix(word: String, ch: char) -> String {
        if word.find(ch) == None {
            return word;
        }
        
        let total_len = word.len();
        let reverse_upto_index = word.find(ch).unwrap();
        let mut output = String::from("");
        
        for i in (0..(reverse_upto_index + 1)).rev() {
            output.push(word.chars().nth(i).unwrap());
        }
        
        for i in (reverse_upto_index + 1)..total_len {
            output.push(word.chars().nth(i).unwrap());
        }
        return output;
    }
}
