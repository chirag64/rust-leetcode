use std::cmp;

impl Solution {
    // Calculate number of even and odd numbers. Return the min of the two
    pub fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
        let mut odds = 0;
        let mut evens = 0;
        for chip in chips {
            if chip % 2 == 0 {
                evens += 1;
            } else {
                odds += 1;
            }
        }
        return cmp::min(evens, odds);
    }
}
