impl Solution {
    // Create a boolean for each character. Loop through the individual characters of the string and mark the booleans true if their corresponding condition gets satisfied
    pub fn strong_password_checker_ii(password: String) -> bool {
        let has_8_chars = (password.len() >= 8);
        let mut has_lowercase_letter = false;
        let mut has_uppercase_letter = false;
        let mut has_digit = false;
        let mut has_special_char = false;
        let mut has_same_adjacent_char = false;
        
        let special_chars = vec![33, 64, 35, 36, 37, 94, 38, 42, 40, 41, 45, 43];
        
        let mut prev_char_i = -1;
        
        for (idx, ch) in password.chars().enumerate() {
            let ch_i = ch as i32;
            if (ch_i >= 97) && (ch_i <= 122) {
                has_lowercase_letter = true;
            }
            if (ch_i >= 65) && (ch_i <= 90) {
                has_uppercase_letter = true;
            }
            if (ch_i >= 48) && (ch_i <= 57) {
                has_digit = true;
            }
            if special_chars.contains(&ch_i) {
                has_special_char = true;
            }
            if idx == 0 {
                prev_char_i = ch_i;
            } else {
                if ch_i == prev_char_i {
                    has_same_adjacent_char = true;
                }
                prev_char_i = ch_i
            }
        }
        return has_8_chars && has_lowercase_letter && has_uppercase_letter && has_digit && has_special_char && !has_same_adjacent_char;
    }
}
