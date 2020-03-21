// Calculate power of all numbers from lo to high and add them in a vector with their key. Then sort the vector based on the power value. Return the k-th item from the vector
impl Solution {
    pub fn get_power(num: i32, pow: i32) -> i32 {
        if num == 1 {
            return pow;
        }
        if num % 2 == 0 {
            return Solution::get_power((num / 2), (pow + 1));
        } else {
            return (Solution::get_power(((num * 3) + 1), (pow + 1)));
        }
    }
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut nums: Vec<Vec<i32>> = Vec::new();
        for i in lo..(hi + 1) {
            nums.push([i, Solution::get_power(i, 0)].to_vec());
        }
        nums.sort_by_key(|a| a[1]);
        return nums[(k - 1) as usize][0];
    }
}
