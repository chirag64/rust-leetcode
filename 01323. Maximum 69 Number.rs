impl Solution {
    // Check digits from left to right. When we find a 9, replace it with 6
    pub fn maximum69_number (mut num: i32) -> i32 {
        // position checks which position we're checking at. 0 is units place, 1 is tens place, and so on...
        let mut position = 6;
        let mut current_digit;
        let mut ten_pow_position;
        while (position > 0) {
            position -= 1;
            ten_pow_position = i32::pow(10, position);
            current_digit = num / ten_pow_position;
            while (current_digit > 9) {
                current_digit %= 10;
            }
            if current_digit == 6 {
                num += 3 * (ten_pow_position);
                break;
            }
        }
        return num;
    }
}
