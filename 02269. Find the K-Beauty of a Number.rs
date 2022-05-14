impl Solution {
    // Convert number to string and keep generating k-sized smaller numbers from it. Count the times num can be divided by those smaller numbers (as long as they aren't zero) and then return it
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let num_str = num.to_string();
        let len = num_str.len();
        let mut k_beauty = 0;
        for i in 0..(len - (k as usize) + 1) {
            let n = &num_str[i..(i + (k as usize))].parse::<i32>().unwrap();
            if (*n != 0) && (num % n == 0) {
                k_beauty += 1;
            }
            
        }
        return k_beauty;
    }
}
