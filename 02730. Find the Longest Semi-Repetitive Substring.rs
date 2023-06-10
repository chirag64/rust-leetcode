use std::cmp::max;

impl Solution {
    // Loop through string and keep counting after how many instances a character is repeated. Maintain these counts in another vector. Keep checking what is the highest total between 2 consecutive values in this list. Return the highest total
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let mut split_sizes = Vec::new();
        let mut current_split_counter = 0;
        let mut largest_continous_split_size = 0;
        let mut prev_char:char = '-';
        for c in s.chars() {
            if prev_char == c {
                split_sizes.push(current_split_counter);
                if split_sizes.len() > 1 {
                    largest_continous_split_size = max(largest_continous_split_size, (split_sizes[split_sizes.len() - 1] + split_sizes[split_sizes.len() - 2]));
                }
                current_split_counter = 0;
            }
            prev_char = c;
            current_split_counter += 1;
        }
        
        split_sizes.push(current_split_counter);
        
        if split_sizes.len() > 1 {
                    largest_continous_split_size = max(largest_continous_split_size, (split_sizes[split_sizes.len() - 1] + split_sizes[split_sizes.len() - 2]));
                } else {
                    largest_continous_split_size = split_sizes[split_sizes.len() - 1];
        }
        
        return (largest_continous_split_size) as i32;
    }
}
