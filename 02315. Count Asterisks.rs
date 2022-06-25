impl Solution {
    // Iterate through the string chars and maintain a boolean that keeps a track of whether we are inside a vertical bar pair. If we are not inside a vertical bar pair, count the asterisk for the final result.
    pub fn count_asterisks(s: String) -> i32 {
        let mut asterisks = 0;
        let mut are_we_inside_bar_pair = false;
        for c in s.chars() {
            if c == '|' {
                are_we_inside_bar_pair = !are_we_inside_bar_pair;
            }
            if c == '*' && !are_we_inside_bar_pair {
                asterisks += 1;
            }
        }
        return asterisks;
    }
}
