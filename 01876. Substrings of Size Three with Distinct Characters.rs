impl Solution {
    // Loop through string and create substrings of size 3 along the way. Compare and make sure each character in substring is unique. If it is add 1 to the result
    pub fn count_good_substrings(s: String) -> i32 {
        let len = s.len();
        let mut substrings = 0;
        
        if len > 2 {
            for c in 0..(len - 2) {
                let first_char = s.chars().nth(c).unwrap();
                let second_char = s.chars().nth(c + 1).unwrap();
                let third_char = s.chars().nth(c + 2).unwrap();

                if (first_char != second_char) && (second_char != third_char) && (third_char != first_char) {
                    substrings += 1;
                } 
            }
        }
        
        return substrings;
    }
}
