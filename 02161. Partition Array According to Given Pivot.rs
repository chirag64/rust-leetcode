impl Solution {
    // Create 3 vectors: one containing values less than pivot, another equal to pivot and third containing values greater than pivot. Then return all 3 vectors concatenated as a single vector
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut less_than_pivot = Vec::new();
        let mut equal_to_pivot = Vec::new();
        let mut more_than_pivot = Vec::new();
        for num in nums {
            if num < pivot {
                less_than_pivot.push(num);
            } else if num > pivot {
                more_than_pivot.push(num);
            } else {
                equal_to_pivot.push(num);
            }
        }
        less_than_pivot.extend(equal_to_pivot);
        less_than_pivot.extend(more_than_pivot);
        return less_than_pivot;
    }
}
