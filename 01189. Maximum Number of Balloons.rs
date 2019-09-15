use std::collections::HashMap;
use std::cmp::min;

impl Solution {
    // Count the number of b-a-l-o-n letters in the 'text' string & then check the minimum count of all these (divide counts of o and n by 2 since they need to appear twice)
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut char_count: HashMap<char, i32> = HashMap::new();
        let mut max_balloons_possible: i32 = std::i32::MAX;
        let mut count;
        
        // Count the number of instances of each letter in the 'text' string
        for char in text.chars() {
            let mut c = char_count.entry(char).or_insert(0);
            *c += 1;
        }
        
        // Check minimum of count of letters b-a-n
        for char in "ban".chars() {
            count = match char_count.get(&char) {
                Some(&count) => count as i32,
                None => 0
            };
            // println!("{}, {}", char, count);
            max_balloons_possible = min(max_balloons_possible, count)
        }
        
        // Divide the count of letters l and o by 2 individually and then check the minimum of the two
        for char in "lo".chars() {
            count = match char_count.get(&char) {
                Some(&count) => (count / 2) as i32,
                None => 0
            };
            // println!("{}, {}", char, count);
            max_balloons_possible = min(max_balloons_possible, count);
        }
        return max_balloons_possible;
    }
}
