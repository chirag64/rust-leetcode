impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut numbers = digits;
        let size = numbers.len();
        
        // If there is only a single digit and it is 9, change it to a two-sized vector with 1, 0
        if (size == 1 && numbers[0] == 9) {
            return [1, 0].to_vec();
        }
        // If last digit is 9, change it to 0 and pass the rest of the vector back to this function recursively
        if (numbers[size - 1] == 9) {
            numbers = Solution::plus_one((&numbers[0..(size - 1)]).to_vec());
            numbers.push(0);
        } else {
            
            // Most normal scenario. Add 0
            numbers[size - 1] += 1;
        }
        return numbers;
    }
}
