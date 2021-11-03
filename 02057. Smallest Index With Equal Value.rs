impl Solution {
    // Loop through nums. Calculate index mod 10 and return index if calculated value matches nums[i]. If there are no matches, return -1
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for (index, num) in nums.iter().enumerate() {
            if index % 10 == (*num as usize) {
                return index as i32;
            }
        }
        return -1;
    }
}
