use std::collections::HashMap;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        // Map integers to their equivalent roman characters
        let v_ints = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let v_romans = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        
        let size = v_ints.len();
        let mut answer = String::new();
        let mut i = 0;
        let mut number = num;
        let mut additions = 0;
        
        loop {
            if (i == size) {
                break;
            }
            
            // If number outputs a quotient when divided by the current number, append the corresponding string of that number 'quotient' no. of times.
            additions = number / v_ints[i];
            if (additions != 0) {
                number %= (additions * v_ints[i]);
                // println!("{}", v_romans[i].repeat(additions as usize));
                answer.push_str(&v_romans[i].repeat(additions as usize));
            }
            i += 1;
        }
        
        return answer;
    }
}
