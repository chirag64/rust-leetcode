use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        
        // Map roman characters to their equivalent integers
        let hm: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000)
        ].iter().cloned().collect();
        
        let s_length = s.len() as usize;
        
        // Convert string into a vector
        let s_arr: Vec<_> = s.chars().collect();
        let mut i: usize = 0;
        
        // Add / subtract in answer variable and then return it
        let mut answer = 0;
        
        loop {
            if (i == s_length) {
                break;
            }
            
            // Check the current character with the next one. If current character's value is smaller than the next one, we add the next one and subtract the current one from the answer. Then we skip the next character from this loop
            if (((i + 1) != s_length) && (hm[&s_arr[i]]) < (hm[&s_arr[i + 1]])) {
                // println!("{} is less than {}", hm[&s_arr[i]], hm[&s_arr[i + 1]]);
                answer += hm[&s_arr[i + 1]] - hm[&s_arr[i]];
                i += 2;                
            } else {
                // Else we simply add the current character
                answer += hm[&s_arr[i]];
                i += 1;
            }
        }
        return answer;
    }
}
