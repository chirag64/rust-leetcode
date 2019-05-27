use std::cmp;

impl Solution {
    pub fn max_satisfied(mut customers: Vec<i32>, mut grumpy: Vec<i32>, x: i32) -> i32 {
        let size = customers.len();
        let mut biggest_grumpy_of_x = 0;
        let mut total_grumpy = 0;
        let mut total_satisfied = 0;
        let mut i = 0;
        loop {
            if (i == size) {
                break;
            }
            
            // Calculate total grumpy customers
            if (grumpy[i] == 1) {
                total_grumpy += customers[i];
            } else {
                // Calculate total satisfied customers and update customers vector to only show grumpy customers
                total_satisfied += customers[i];
                customers[i] = 0;
            }
            
            // Calculate biggest total of x grumpy customers till now
            if (i as i32 > (x - 2)) {
                let mut j: i32 = (i as i32 - (x - 1));
                let mut total_of_x = 0;
                loop {
                    if (j as usize == (i + 1)) {
                        break;
                    }
                    total_of_x += customers[j as usize];
                    j += 1;
                }
                biggest_grumpy_of_x = cmp::max(biggest_grumpy_of_x, total_of_x);
                
            }
            i += 1;
        }
        /*println!("{:?}", customers);
        println!("{}", biggest_grumpy_of_x);
        println!("{}", total_grumpy);
        println!("{}", total_satisfied);*/  
        
        // Return total satisfied + biggest grumpy of x items
        return total_satisfied + biggest_grumpy_of_x;
        
    }
}
