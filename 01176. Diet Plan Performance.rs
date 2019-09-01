impl Solution {
    pub fn diet_plan_performance(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut points = 0;
        let total_days = calories.len();
        let mut calories_to_count = 0;
        
        loop {
            if ((i + k) > (total_days) as i32) {
                break;
            }
            
            // Add up calories from day j to (j + k). Then check if it is more than upper or less than lower and calculate points accordingly. Keep moving j with i
            j = i;
            calories_to_count = 0;
            loop {
                if (j == (i + k)) {
                    break;
                }
                calories_to_count += calories[(j) as usize];
                j += 1;
            }
            if (calories_to_count < lower) {
                println!("l {}, {}", calories_to_count, lower);
                points -= 1;
            } else if (calories_to_count > upper) {
                println!("u {}, {}", calories_to_count, upper);
                points += 1;
            }
            i += 1;
        }
        return points;
    }
}
