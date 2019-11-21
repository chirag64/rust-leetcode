use std::cmp;

impl Solution {
    
    // Add all numbers and store these 4 numbers: smallest_mod_1, second_smallest_mod_1, smallest_mod_2, second_smallest_mod_2. If sum % 3 == 0, return sum. If sum % 3 == 1, return max of (sum - smallest_mod_1) and (sum - (smallest_mod_2 + second_smallest_mod_2)). If sum % 3 == 2, return max of (sum - smallest_mod_2) and (sum - (smallest_mod_1 + second_smallest_mod_1))
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut smallest_mod_1 = 4000;
        let mut second_smallest_mod_1 = 4000;
        let mut smallest_mod_2 = 4000;
        let mut second_smallest_mod_2 = 4000;
        
        for i in nums {
            sum += i;
            
            if (i % 3 == 1 && i <= second_smallest_mod_1) {
                if i <= smallest_mod_1 {
                    second_smallest_mod_1 = smallest_mod_1;
                    smallest_mod_1 = i;
                } else {
                    second_smallest_mod_1 = i;
                }
            }
            
            if (i % 3 == 2 && i <= second_smallest_mod_2) {
                if i < smallest_mod_2 {
                    second_smallest_mod_2 = smallest_mod_2;
                    smallest_mod_2 = i;
                } else {
                    second_smallest_mod_2 = i;
                }
            }
        }
        
        if sum % 3 == 0 {
            return sum;
        } else if sum % 3 == 1 {
            return cmp::max(sum - (smallest_mod_2 + second_smallest_mod_2), sum - smallest_mod_1);
        } else {
            return cmp::max(sum - (smallest_mod_1 + second_smallest_mod_1), sum - smallest_mod_2);
        }
    }
}
