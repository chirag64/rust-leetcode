use std::collections::HashMap;

impl Solution {
    // Count instances of each num. Then loop through those counts and find out if any number + k exists in that count instance. If it does, multiply their frequencies and add them in the output (if 1 occurs twice and 2 occurs thrice, they can be paired 2 x 3 = 6 number of times)
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut output = 0;
        let mut map = HashMap::new();

        for num in &nums {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }
        
        for (num, num_count) in &map {
            if map.contains_key(&(**num + k)) {
                output += (*num_count * (map.get(&(**num + k)).unwrap()));
            }
        }
        
        return output;
    }
}
