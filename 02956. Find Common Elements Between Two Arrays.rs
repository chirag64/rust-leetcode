use std::collections::HashSet;

impl Solution {
    // Create HashSet for nums1 and nums2. Then loop through nums1 and nums2 to check how many of them are available in the HashSets.
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        let mut result = vec![0, 0];
        
        for num in &nums1 {
            let count = set1.insert((*num).to_string());
        }
        
        for num in &nums2 {
            let count = set2.insert((*num).to_string());
        }
        
        for num in &nums1 {
            if set2.contains(&((*num).to_string())) {
                result[0] += 1;
            }
        }
        
        for num in &nums2 {
            if set1.contains(&((*num).to_string())) {
                result[1] += 1;
            }
        }
        
        return result;
    }
}
