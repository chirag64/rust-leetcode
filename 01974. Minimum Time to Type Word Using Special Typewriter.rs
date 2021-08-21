impl Solution {
    // Keep a tracker variable for seconds. Add 1 second for every character in word. Keep track of previous character ASCII value. Initially it will be 1 and set to a. Then calculate the absolute difference in ASCII value of current character & previous character. If difference is greater than 13, that means we need to go counter-clockwise (26 - difference). Return the final number of seconds
    pub fn min_time_to_type(word: String) -> i32 {
        let mut seconds = word.len();
        let mut prev_char = 1;
        for ch in 0..word.len() {
            let current_char = word.chars().nth(ch).unwrap() as usize - 96;
            let mut difference = i32::abs((current_char - prev_char) as i32) as usize;
            
            if difference > 13 {
                difference = 26 - difference;
            }
            
            seconds += difference;
            prev_char = current_char;
        }
        return seconds as i32;
    }
}
