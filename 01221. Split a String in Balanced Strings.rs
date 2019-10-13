impl Solution {
    // Loop through string, keep counting number of Ls & Rs. If at any point, both count are same, reset those counters and add 1 to master counter1
    pub fn balanced_string_split(s: String) -> i32 {
        let mut counter = 0;
        let mut l_counter = 0;
        let mut r_counter = 0;
        for c in s.chars() {
            if c == 'L' {
                l_counter += 1;
            } else {
                r_counter += 1;
            }
            if l_counter == r_counter {
                l_counter = 0;
                r_counter = 0;
                counter += 1;
            }
        }
        return counter;
    }
}
