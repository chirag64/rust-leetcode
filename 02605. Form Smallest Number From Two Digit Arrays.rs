use std::cmp::min;

impl Solution {
    // Run a nested loop over both vectors. Initialize smallest_num as the biggest i32 value. If both numbers in the loop iteration are same, smallest_num should be the smallest of: smallest_num and the number in iteration. If both numbers in the loop iteration are different, smallest_num should be the smallest of: smallest_num and n1n2 and n2n1.
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut smallest_num = i32::MAX;
        for n1 in &nums1 {
            for n2 in &nums2 {
                if n1 == n2 {
                    smallest_num = min(smallest_num, *n1);
                } else {
                    smallest_num = min(min(((*n1 * 10) + *n2), ((*n2 * 10) + *n1)), smallest_num);
                }
            }
        }
        return smallest_num;
    }
}
