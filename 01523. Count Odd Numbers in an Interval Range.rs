impl Solution {
    // Calculate difference to find number of numbers between the two. Half these numbers are odd. If difference is odd, check if high is an odd number. If it is, add 1
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let numbers_in_between = (high - (low - 1));
        let mut odds_in_between = numbers_in_between / 2;
        if (numbers_in_between % 2 != 0) && (high % 2 != 0) {
            odds_in_between += 1;
        }
        return odds_in_between;        
    }
}
