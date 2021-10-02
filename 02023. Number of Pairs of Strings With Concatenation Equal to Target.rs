impl Solution {
    // Loop through nums twice. If concatenating the strings found in both loops matches target, add count
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let len = nums.len();
        let mut matches_count = 0;
        
        for i in 0..len {
            for j in 0..len {
                
                if i != j {
                    if nums[i].to_owned() + &nums[j].to_owned() == target {
                        matches_count += 1;
                    }
                }
            }
        }
        return matches_count;
    }
}
