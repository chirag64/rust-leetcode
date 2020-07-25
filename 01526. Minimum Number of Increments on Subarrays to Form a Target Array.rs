use std::cmp::max;

impl Solution {
    // Loop through the vector while comparing adjacent numbers. If right number is greater than left number, count the difference and add it in result
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let len = target.len();
        let mut i = 1;
        let mut number_of_ups = target[0];
        while i != len {
            if target[i] > target[i - 1] {
                number_of_ups += (target[i] - target[i - 1]);
            }
            i += 1;
        }
        return number_of_ups;
    }
}
