impl Solution {
    // Loop through ops, follow the instructions in a if-else blocks to create a final vec. Then loop through the final vec to calculate the result sum
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut final_vec = Vec::new();
        let mut sum = 0;
        
        for op in ops {
            let len = final_vec.len();
            
            if op == "C" {
                final_vec.pop();
            } else if op == "D" {
                final_vec.push(final_vec[len - 1] * 2);
            } else if op == "+" {
                final_vec.push(final_vec[len - 1] + final_vec[len - 2]);
            } else {
                final_vec.push(op.parse::<i32>().unwrap());
            }
        }
        
        for num in final_vec {
            sum += num;
        }
        
        return sum;
    }
}
