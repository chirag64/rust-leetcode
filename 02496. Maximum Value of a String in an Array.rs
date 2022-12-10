use std::cmp::max;

impl Solution {
    // Loop through the vector of strings. Keep a track of whether we found a non-digit in each iterated string. If we did, compare its length with the highest value we found so far. If we didn't find any non-digit in the iterated string, convert it into a i32 and compare its value with the highest value we found so far.
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        let mut result = 0;
        
        for string_iterator in strs {
            let mut found_a_non_digit = false;
            for ch in string_iterator.chars() { 
                let ascii_value = ch as usize;
                
                // The ASCII value of English numbers is 48 to 57
                if ascii_value < 48 || ascii_value > 57 {
                    result = max(result, (string_iterator.len() as i32));
                    found_a_non_digit = true;
                    break;
                }
            }
            
            if !found_a_non_digit {                
                result = max(result, (string_iterator.parse::<i32>().unwrap()));
            }
        }
        return result;
    }
}
