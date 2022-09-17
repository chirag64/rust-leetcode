use std::cmp::max;
use std::cmp::min;

impl Solution {
    // Convert string vectors to individual int values. Then find the last arrival day among both along with first leaving day for both. Then loop through the months from last arrival day to first leaving day, taking care that only pending days are counted from first and last months
    pub fn count_days_together(arrive_alice: String, leave_alice: String, arrive_bob: String, leave_bob: String) -> i32 {
        let days_in_months: Vec<i32> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let arrive_alice_split_obj: Vec<&str> = arrive_alice.split("-").collect();
        let leave_alice_split_obj: Vec<&str> = leave_alice.split("-").collect();
        let arrive_bob_split_obj: Vec<&str> = arrive_bob.split("-").collect();
        let leave_bob_split_obj: Vec<&str> = leave_bob.split("-").collect();
        
        let arrive_alice_obj = [arrive_alice_split_obj[0].parse::<i32>().unwrap(), arrive_alice_split_obj[1].parse::<i32>().unwrap()];
        let leave_alice_obj = [leave_alice_split_obj[0].parse::<i32>().unwrap(), leave_alice_split_obj[1].parse::<i32>().unwrap()];
        let arrive_bob_obj = [arrive_bob_split_obj[0].parse::<i32>().unwrap(), arrive_bob_split_obj[1].parse::<i32>().unwrap()];
        let leave_bob_obj = [leave_bob_split_obj[0].parse::<i32>().unwrap(), leave_bob_split_obj[1].parse::<i32>().unwrap()];
        
        let arrive_common_month = if arrive_alice_obj[0] > arrive_bob_obj[0] {
            arrive_alice_obj[0]
        } else {
            arrive_bob_obj[0]
        };
        
        let arrive_common_date = if arrive_alice_obj[0] > arrive_bob_obj[0] {
            arrive_alice_obj[1]
        } else if arrive_alice_obj[0] < arrive_bob_obj[0] {
            arrive_bob_obj[1]
        } else {
            max(arrive_alice_obj[1], arrive_bob_obj[1])
        };
        
        let leave_common_month = if leave_alice_obj[0] < leave_bob_obj[0] {
            leave_alice_obj[0]
        } else {
            leave_bob_obj[0]
        };

        let leave_common_date = if leave_alice_obj[0] < leave_bob_obj[0] {
            leave_alice_obj[1]
        } else if leave_alice_obj[0] > leave_bob_obj[0] {
            leave_bob_obj[1]
        } else {
            min(leave_alice_obj[1], leave_bob_obj[1])
        };
        
        let mut result = 0;
        for current_month in arrive_common_month..=leave_common_month {
            if current_month == arrive_common_month {
                if current_month == leave_common_month {
                    result += max((leave_common_date - arrive_common_date + 1), 0);
                } else {
                    result += days_in_months[(current_month as usize) - 1] - arrive_common_date + 1; 
                }
                
            } else if current_month == leave_common_month {
                result += leave_common_date;
            } else {
                result += (days_in_months[(current_month as usize) - 1]);
            }
        }

        return result;
    }
}
