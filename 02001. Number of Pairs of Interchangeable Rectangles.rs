use std::collections::HashMap;

impl Solution {
    // Loop through rectangles and calculate the width / height ratio. Maintain a hashmap of these ratios with their counts. Then loop through this hashmap and every ratio that occurs n number of times can be interchanged into (n x (n - 1)) / 2 times which needs to be added to the final output
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        
        let mut ratio_map = HashMap::new();
        let mut output = 0i64;
        
        for rectangle in rectangles {
            let ratio = (rectangle[0] as f64) / (rectangle[1] as f64);
            let count = ratio_map.entry(ratio.to_string()).or_insert(0);
            *count += 1;
        }
        
        for (ratio, ratio_count) in &ratio_map {
            output += (ratio_count * (ratio_count - 1)) / 2;
        }
        
        return output;
    }
}
