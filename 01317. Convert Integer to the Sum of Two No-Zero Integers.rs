impl Solution {
    // Brute force. Start first number a with 0 and b with n. Keep adding 1 to a and subtracting 1 from b and check if they are numbers without 0
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut a = 0;
        let mut b = n;
        let mut found_zero = true;
        while(found_zero) {
            found_zero = false;
            a += 1;
            b -= 1;
            for i in a.to_string().chars() {
                if (i == '0') {
                    found_zero = true;
                }
            }
            for i in b.to_string().chars() {
                if (i == '0') {
                    found_zero = true;
                }
            }
        }
        return [a, b].to_vec();
    }
}
