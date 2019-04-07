use std::num::ParseIntError;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        
        // Define a function that will handle errors for us and return a reverse number or 0 if i32 is bigger than max
        fn reverseNumber(x: i32) -> Result<i32, i32> {
           match x.to_string().chars().rev().collect::<String>().parse::<i32>() {
                Ok(n) => Ok(n),
             Err(err) => Err(0),
            }
        }
        
        // If x is negative, make it positive and then reverse it and then make it negative again
        if (x < 0) {
            let mut x = x * -1;
            match reverseNumber(x) {
                Ok(n) => return n * -1,
                Err(err) => return err,
            }
        } else {
            // Else convert into a string and reverse it
            match reverseNumber(x) {
                Ok(n) => return n,
                Err(err) => return err,
            }
        }
    }
}
