impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // Convert to string & compare with reverse
        return x.to_string() == x.to_string().chars().rev().collect::<String>();
    }
}
