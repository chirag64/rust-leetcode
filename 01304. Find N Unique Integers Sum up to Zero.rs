impl Solution {
    // Generate an array from -(n / 2) to (n / 2)
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut arr: Vec<i32> = Vec::new();
        for i in (-(n / 2)..((n / 2) + 1)) {
            if !(n % 2 == 0 && i == 0) {
                arr.push(i);
            }
        }
        return arr;
    }
}
