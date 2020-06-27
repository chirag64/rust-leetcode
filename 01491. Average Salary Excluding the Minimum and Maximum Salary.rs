impl Solution {
    // Add up all salaries & keep track of min & max numbers. Then subtract min & max from sum and then calculate avg
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut result: f64 = 0.0;
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        for s in &salary {
            result += (*s as f64);
            min = std::cmp::min(min, *s);
            max = std::cmp::max(max, *s);
        }
        result -= (min + max) as f64;
        result /= ((salary.len() - 2) as f64);
        return result;
    }
}
