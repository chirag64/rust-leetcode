use std::collections::HashSet;
impl Solution {
    // Use nested loop to track numerator & denominator. Divide the 2 numbers as floats and convert result in string. If the string does not exist in our hashset (i.e. we haven't encountered this fraction before), add it to the hashset & create a string out of it to add to our result vector
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let mut i: u8 = 1;
        let mut set = HashSet::new();
        let mut result = Vec::new();
        while (i <= n as u8) {
            let mut j: u8 = 1;
            while (j != i) {
                let division_result = ((i as f64) / (j as f64)).to_string();
                if !set.contains(&division_result) {
                    set.insert(division_result);
                    let v = vec![j.to_string(), "/".to_string() , i.to_string()];
                    let s: String = v.join("");
                    result.push(s);
                }
                j += 1;
            }
            i += 1;
        }
        return result;
    }
}
