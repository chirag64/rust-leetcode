use std::cmp::min;
use std::cmp::max;

impl Solution {
    // Generate a vector calculated with the values of differences, start it with 0. Keep a track of the biggest and smallest numbers in this vector. After the vector is generated, calculate the difference between the biggest and smallest numbers of this vector with the difference in upper and lower. The difference between these two values is the different combinations that the hidden vector can exist. If this difference comes out negative, that means no solution for the hidden vector is possible
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut current_num: i64 = 0;
        let mut smallest_num: i64 = 0;
        let mut biggest_num: i64 = 0;
        
        for d in differences {
            current_num += d as i64;
            smallest_num = min(smallest_num, current_num);
            biggest_num = max(biggest_num, current_num);
        }
        
        let output = (upper - lower) - ((biggest_num - smallest_num) as i32) + 1;
        
        if output > -1 {
            return output;
        }
        
        return 0;
    }
}
