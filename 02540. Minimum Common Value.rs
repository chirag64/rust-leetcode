use std::collections::HashSet;

impl Solution {
    // Create a HashSet for nums1. Then loop through num2 and keep a track of the smallest number from num2 that exists in num1. Initialize smallest_num variable to -1 so that if there is no match, it returns -1
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums1_hashset = HashSet::new();
        let mut smallest_num = -1;

        for num in nums1 {
            nums1_hashset.insert(num.to_string());
        }
        
        for num in nums2 {
            if nums1_hashset.contains(&num.to_string()) && (smallest_num == -1 || smallest_num > num) {
                smallest_num = num;
            }
        }
        
        return smallest_num;
    }
}
