use std::cmp::max;

impl Solution {
    // Loop through nums and keep track of the largest and smallest nums and how frequently they occur. Then return the length of nums subtracted by the frequencies of largest and smallest numbers
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let mut smallest = i32::MAX;
        let mut largest = i32::MIN;
        let mut smallest_freq = 0;
        let mut largest_freq = 0;
        
        for num in &nums {
            if smallest > *num {
                smallest = *num;
                smallest_freq = 1;
            } else if smallest == *num {
                smallest_freq += 1;
            }
            
            if largest < *num {
                largest = *num;
                largest_freq = 1;
            } else if largest == *num {
                largest_freq += 1;
            }
        }
        return max((nums.len() as i32 - smallest_freq - largest_freq), 0)
    }
}
