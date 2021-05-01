use std::char::from_u32;

impl Solution {
    // Convert characters to ASCII value and digit characters to u32. Then add them up and convert the result back to a char which can be added to the result string. If result string size is not same as input string, that means last character of input string is a character that need not be changed. Simply append it to result string
    pub fn replace_digits(s: String) -> String {
        let length = s.len();
        let mut result = String::new();
        
        for i in 0..(length / 2) {
            
            let ch = s.chars().nth(i * 2).unwrap();
            let ch_ascii = ch as u32;
            let digit = (s.chars().nth((i * 2) + 1).unwrap() as u32) - 48;
            
            result.push(ch);
            result.push(from_u32(ch_ascii + digit).unwrap());
            
        }
        if length != result.len() {
            result.push(s.chars().nth(length - 1).unwrap());
        }
        return result;
    }
}
