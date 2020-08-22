use std::cmp::min;

// Convert number to string, break down string into a vector substrings of 3 or less. Then join them back as a string with a . separator
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let s = n.to_string().chars().rev().collect::<String>();
        let size = s.len();
        let mut vec: Vec<String> = Vec::new();
        let mut string_processed = false;
        let mut i = 0;
        
        while !string_processed {
            let start = i * 3;
            let end = min((i * 3) + 3, size);
            let mut num_str = &s[start..end];
            vec.insert(0, num_str.chars().rev().collect());
            if end == size {
                string_processed = true;
            }
            i += 1;
        }
        return vec.join(".");
    }
}
