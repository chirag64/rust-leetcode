impl Solution {
    // Simple method for tracking day_number and money on that day. If day_number is a Monday, re-calculate money_that_day by divinding day number by 7, else simply
    pub fn total_money(n: i32) -> i32 {
        let mut day_number = 0;
        let mut money_that_day = 1;
        let mut result = 0;
        while day_number < n {
            if day_number % 7 == 0 {
                money_that_day = (day_number / 7) + 1;
            }
            result += money_that_day;
            day_number += 1;
            money_that_day += 1;
        }
        return result;
    }
}
