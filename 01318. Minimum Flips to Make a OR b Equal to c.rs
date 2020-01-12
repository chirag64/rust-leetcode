use std::cmp;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        
        // Convert all numbers to binary strings
        let mut a_str = format!("{:b}", a);
        let mut b_str = format!("{:b}", b);
        let mut c_str = format!("{:b}", c);
        
        let mut a_ch;
        let mut b_ch;
        let mut c_ch;
        
        let mut flips = 0;
        let biggest_string_size = cmp::max(cmp::max(a_str.len(), b_str.len()), c_str.len());
        
        // Convert all binary strings to equal lengths, prepend 0s at the start
        while (a_str.len() != biggest_string_size) {
            a_str = format!("0{}", a_str);
        }
        while (b_str.len() != biggest_string_size) {
            b_str = format!("0{}", b_str);
        }
        while (c_str.len() != biggest_string_size) {
            c_str = format!("0{}", c_str);
        }
        
        for i in 0..biggest_string_size {
            a_ch = a_str.chars().nth(i).unwrap();
            b_ch = b_str.chars().nth(i).unwrap();
            c_ch = c_str.chars().nth(i).unwrap();
            
            // If c is 0, count number of 1s that need to be flipped
            if(c_ch == '0') {
                if (a_ch == '1') {
                    flips += 1;
                }
                if (b_ch == '1') {
                    flips += 1;
                }
            } else if (c_ch == '1') {
                // If c is 1, make sure both a and b are not 0. If they are, flip 1
                if ((a_ch == '0' && b_ch == '0')) {
                    flips += 1;
                }
            }
        }
        return flips;
    }
}
