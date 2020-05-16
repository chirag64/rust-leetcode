use std::cmp;
impl Solution {
    // Maintain a max counter and current counter. Loop through the string & compare adjacent characters. If they are same, increment current counter, else update max counter & reset current counter
    pub fn max_power(s: String) -> i32 {
        let size = s.len();
        let mut i = 0;
        let mut largest_substring_size = 1;
        let mut current_substring_size = 1;
        while i != (size - 1) {
            let current_char = s.chars().nth(i).unwrap();
            let next_char = s.chars().nth(i + 1).unwrap();
            if current_char == next_char {
                current_substring_size += 1;
            } else {
                largest_substring_size = cmp::max(largest_substring_size, current_substring_size);                
                current_substring_size = 1;
            }
            i += 1;
        }
        largest_substring_size = cmp::max(largest_substring_size, current_substring_size);
        return largest_substring_size;
    }
}
