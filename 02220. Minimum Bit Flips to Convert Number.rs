impl Solution {
    // Convert both integers to strings of binary number format. If one of the strings is smaller than the other, prepend the appropriate number of zeroes to its start so that both the strings have same size. Then compare both strings character by character, returning the number of differences (or flips needed)
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let mut differences = 0;
        let mut start_str = format!("{:b}", start);
        let mut goal_str = format!("{:b}", goal);
        let start_str_len = start_str.len();
        let goal_str_len = goal_str.len();
        if start_str_len > goal_str_len {
            goal_str =  format!("{}{}", "0".repeat(start_str_len - goal_str_len), goal_str)
        } else if goal_str_len > start_str_len {
            start_str =  format!("{}{}", "0".repeat(goal_str_len - start_str_len), start_str)
        }
        
        for i in 0..start_str.len() {
            if start_str.chars().nth(i).unwrap() != goal_str.chars().nth(i).unwrap() {
                differences += 1;
            }
        }
        
        return differences;
    }
}
