impl Solution {
    // Loop from 1 to n. Keep a count of which one of these are factors of n. Once you find the k-th factor, return it
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut j = 0;
        for i in 1..(n + 1) {
            if n % i == 0 {
                j += 1;
                if (j == k) {
                    return i;
                }
            }
        }
        return -1;
    }
}
