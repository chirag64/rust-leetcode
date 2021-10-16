impl Solution {
    // Sort students and seats in ascending order (of their position) and then seat them in the same order by simply absoluting the difference. This works because the number of seats and students is the same. So loop through both the sorted vectors and then keep adding the absolute difference in the final result
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        let mut result = 0;
        let len = seats.len();
        
        seats.sort();
        students.sort();
        
        for i in 0..len {
            result += (seats[i] - students[i]).abs();
        }
        
        return result;
    }
}
