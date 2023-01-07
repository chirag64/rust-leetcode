impl Solution {
    // Initialize result as 'Neither'. If any dimensions or volume are greater than threshold, update result to Bulky. If mass is greater than threshold, update result based on whether it was already Bulky or not. Return the final result
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let mut result = "Neither";
        if (length >= 10000) || (width >= 10000) || (height >= 10000) || ((length as i64) * (width as i64) * (height as i64) >= 1000000000) {
            result = "Bulky"
        }
        if mass >= 100 {
            if result == "Neither" {
                result = "Heavy";
            } else {
                result = "Both";
            }
        }
        return result.to_string();
    }
}
