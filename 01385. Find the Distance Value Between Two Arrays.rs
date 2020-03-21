// Iterate through both loops as nested loops. If difference between both elements in each loop is less than or equal to d, do not add 1 to count, else add 1. Return count
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut count = 0;
        let mut always_greater = true;
        for i in &arr1 {
            always_greater = true;
            for j in &arr2 {
                if (j - i).abs() <= d {
                    always_greater = false;
                }
            }
            if always_greater {
                count += 1;
            }
        }
        return count;
    }
}
