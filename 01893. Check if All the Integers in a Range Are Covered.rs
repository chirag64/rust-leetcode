use std::collections::HashSet;

impl Solution {
    // Create a HashSet from left to right. Then iterate through the ranges and remove those elements from the HashSet. At the end, if HashSet is empty, all integers were covered, else they weren't
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut hs = HashSet::new();
        
        for i in left..(right + 1) {
            hs.insert(i);
        }
        
        for range in ranges {
            for j in range[0]..(range[1] + 1) {
                hs.remove(&j);
            }
        }
        
        return match hs.len() {
            0 => true,
            _ => false
        }
    }
}