use std::cmp::max;

impl Solution {
    // Consider no penalty as a profit. Calculate profit after each hour and keep track of the hour when maximum profit was accumulated. Return that hour
    pub fn best_closing_time(customers: String) -> i32 {
        let mut profit = 0;
        let mut max_profit = 0;
        let mut best_hour_to_close = -1;
        for (i, c) in customers.chars().enumerate() {
            if c == 'Y' {
                profit += 1;
            } else {
                profit -= 1;
            }
            if profit > max_profit {
                max_profit = profit;
                best_hour_to_close = (i as i32);
            }
        }
        return best_hour_to_close + 1;
    }
}
