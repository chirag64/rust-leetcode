impl Solution {
    // Sort the nums vector. Then create 2 markers that start from end, one for perm and one for nums. If perm is greater than nums, then move both markers, else only move nums marker. The result is the number of combinations where perm is greater than nums
    pub fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;
        nums.sort();
        let mut i = nums.len() - 1;
        let mut j = nums.len() - 1;
        
        loop {
            if i != j {
                if nums[i] > nums[j] {
                    result += 1;
                    i -= 1;
                }
            }
            if j == 0 {
                break;
            }
            j -= 1;
        }
        return result;
    }
}
