/*
  Runtime: 4 ms, faster than 26.72% of Rust online submissions for Remove Duplicates from Sorted Array.
  Memory Usage: 2.8 MB, less than 100.00% of Rust online submissions for Remove Duplicates from Sorted Array.
*/

impl Solution {
    // Maintain 2 markers: i and j that start the vector from position 0 and 1 resp. Then every time j's number is higher than i, move both pointers by 1, else only move j's pointer
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let size = nums.len();
        let mut i = 0;
        let mut j = 1;
        
        // If vector is blank, return 0
        if (size == 0) {
            return 0;
        }
        
        // Loop through as long as i is not pointing to last element and j is not exceeding the vector index
        while (((i + 1) < size) && (j < size)) {
            // Loop through if j's number is not larger than i's
            while ((j + 1) < size && (nums[j] <= nums[i])) {
                // If j is pointing to a number same or smaller than i, move j to next element
                j += 1;
            }
            if (((i + 1) < size) && (j < size) && (nums[j] > nums[i])) {
                // If j is not adjacent to i, make i's next number as j's number
                if ((j - i) > 1) {
                    nums[i + 1] = nums[j];
                }
                i += 1;
            }
            j += 1;
        }
        // If i is not the last element and i's number is less than the last number, increment i by 1. If we don't do this, i will never reach the last element. This only happens when all numbers are unique
        if (((i + 2) == size) && (nums[i] < nums[size - 1])) {
            i += 1;
        }
        return (i + 1) as i32;
    }
}
