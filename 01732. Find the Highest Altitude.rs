use std::cmp;

impl Solution {
    // Keep re-calculating current altitude by adding the net gain. Keep track of the highest gain while calculating each gain
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut current_gain = 0;
        let mut highest_gain = 0;
        
        for i in gain {
            current_gain += i;
            highest_gain = cmp::max(highest_gain, current_gain);
        }
        return highest_gain;
    }
}
