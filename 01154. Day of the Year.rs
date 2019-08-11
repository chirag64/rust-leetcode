impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let days_in_months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let split_string: Vec<&str> = date.split('-').collect();
        let year = split_string[0].parse::<i32>().unwrap();
        let month = split_string[1].parse::<i32>().unwrap();
        let date = split_string[2].parse::<i32>().unwrap();
        let mut i: i32 = 1;
        let mut totalDays = 0;
        // Hard code days of month in each year and add accordingly. Add 1 if leap year
        loop {
            if (i == month) {
                break;
            }
            totalDays += days_in_months[(i - 1) as usize];
            i += 1;
        }
        totalDays += date;
        if (((year % 400 == 0) || (year % 100 != 0 && year % 4 == 0)) && (month > 2)) {            
            totalDays += 1;
        }
        return totalDays;
    }
}
