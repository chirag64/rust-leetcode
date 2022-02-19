impl Solution {
    // If num is divisible by 3, divide it by 3 to get the middle number. Then return a vector with first number (1 less than middle number), middle number and last number (1 more than middle number). If num isn't divisible by 3, return empty vector
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 == 0 {
            let middle_num = num / 3;
            return [middle_num - 1, middle_num, middle_num + 1].to_vec();
        } else {
            return [].to_vec();
        }
    }
}
