impl Solution {
    // Loop through details vector. During each iteration, extract the age-specific strings and convert them into integer. Post conversion, check if they are greater than 60. Return the total count
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut result = 0;
        for detail in details {
            let age = &detail[11..13].parse::<i32>().unwrap();
            if *age > 60 {
                result += 1;
            }
        }
        return result;
    }
}
