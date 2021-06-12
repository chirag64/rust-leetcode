impl Solution {
    // Iterate through the chalk vector and keep calculating the chalk used and chalk remaining. If at the end, there's still chalk remaining, calculate modulus of initial total chalk and chalk used. No matter how many rounds they go through, this is the number of chalk that will be left. Then repeat the process. If at any point of time, chalk needed is greater than chalk remaining, return index of the student
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut chalk_used = 0;
        let mut chalk_remaining = k;
        
        for (idx, i) in chalk.iter().enumerate() {
            chalk_used += *i;
            if chalk_remaining < *i {
                return idx as i32;
            } else {
                chalk_remaining -= *i;
            }
        }
        
        chalk_remaining = k % chalk_used;
        
        loop {
            for (idx, i) in chalk.iter().enumerate() {
                chalk_used += *i;
                if chalk_remaining < *i {
                    return idx as i32;
                } else {
                    chalk_remaining -= *i;
                }
            }
        }
    }
}
