impl Solution {
    // If n is 1, return 0. If n is divisible by 2, return the quotient after dividing it by 2. Else return n
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        if n % 2 == 0 {
            return n / 2;
        }
        return n;
    }
}
