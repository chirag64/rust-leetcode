use std::cmp;


// Split the sentence into words. Calculate size of output vector's each element since we'll have to add spaces where words are smaller. Loop through the words and add them in the output vector
impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let mut iter = s.split_whitespace();
        let mut current_iter = iter.next();
        let mut words = Vec::new();
        let mut output = Vec::new();
        let mut output_sizes = Vec::new();
        
        // Split the sentence into words
        while(current_iter != None) {
            words.push(current_iter.unwrap());
            current_iter = iter.next();
        }
        
        // Calculate sizes of output vector's each element since we'll have to add spaces where words are smaller
        for s in words.iter().rev() {
            if output_sizes.len() == 0 {
                output_sizes.push(s.len());
            } else {
                output_sizes.insert(0, cmp::max(output_sizes[0], s.len()));
            }
        }
        
        // Loop through the words and add them in the output vector
        for (idx, s) in words.iter().enumerate() {
            for inner_idx in 0..output_sizes[idx] {
                if (output.len() <= inner_idx) {
                    output.push(String::new());
                }
                let mut current_char;
                if (s.len() <= inner_idx) {
                    current_char = ' ';
                } else {
                    current_char = s.chars().nth(inner_idx).unwrap();
                }
                output[inner_idx] = format!("{}{}", output[inner_idx], current_char);
            }
        }
        return output;
    }
}
